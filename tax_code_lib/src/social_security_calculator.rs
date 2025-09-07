use std::collections::HashMap;
use serde::Deserialize;
use crate::FilingStatus;

const FEDERAL_SS_JSON: &str = include_str!("taxable_social_security.json");
const STATE_SS_JSON: &str = include_str!("social_security_taxes.json");
const YEARLY_LIMIT_INCREASE_RATE: f64 = 0.02; // 2.0%

#[derive(Debug, Deserialize, Clone)]
struct SocialSecurityBracket {
    rate: f64,
    upper_bound: Option<f64>,
    #[serde(default)]
    deduction: Option<f64>,
}

// Type aliases for complex nested structures
type FederalData = HashMap<String, HashMap<String, Vec<SocialSecurityBracket>>>;
type StateYearData = HashMap<String, HashMap<String, Vec<SocialSecurityBracket>>>;
type StateAgeData = HashMap<String, StateYearData>;
type StateData = HashMap<String, StateAgeData>;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum FilingStatusValue {
    Brackets(Vec<SocialSecurityBracket>),
    Federal(String), // For states that use "federal"
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum AgeData {
    Direct(HashMap<String, FilingStatusValue>), // "64": { "Single": [...] }
    Wrapped(Vec<HashMap<String, FilingStatusValue>>), // "65": [{ "Single": [...] }]
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum StateYearDataEnum {
    ByFilingStatus(HashMap<String, FilingStatusValue>),
    ByAge(HashMap<String, AgeData>),
}

type StateRawData = HashMap<String, HashMap<String, StateYearDataEnum>>;

pub struct SocialSecurityTaxCalculator {
    federal_data: HashMap<i32, HashMap<String, Vec<SocialSecurityBracket>>>,
    state_data: HashMap<String, HashMap<i32, StateYearDataEnum>>,
}

impl SocialSecurityTaxCalculator {
    pub fn load() -> Result<Self, String> {
        // Load federal data
        let federal_parsed: FederalData = serde_json::from_str(FEDERAL_SS_JSON)
            .map_err(|e| format!("Failed to parse taxable_social_security.json: {}", e))?;

        let mut federal_data = HashMap::new();
        for (year_str, filing_status_map) in federal_parsed.into_iter() {
            if let Ok(year) = year_str.parse::<i32>() {
                federal_data.insert(year, filing_status_map);
            }
        }

        // Load state data  
        let state_parsed: StateRawData = serde_json::from_str(STATE_SS_JSON)
            .map_err(|e| format!("Failed to parse social_security_taxes.json: {}", e))?;

        let mut state_data = HashMap::new();
        for (state, years_map) in state_parsed.into_iter() {
            let mut inner = HashMap::new();
            for (year_str, year_data) in years_map.into_iter() {
                if let Ok(year) = year_str.parse::<i32>() {
                    inner.insert(year, year_data);
                }
            }
            state_data.insert(state.to_uppercase(), inner);
        }

        Ok(Self {
            federal_data,
            state_data,
        })
    }

    fn status_key(filing_status: &FilingStatus) -> &'static str {
        match filing_status {
            FilingStatus::Single => "Single",
            FilingStatus::MarriedFilingSeparately => "MarriedFilingSeparately",
            FilingStatus::MarriedFilingJointly => "MarriedFilingJointly",
            FilingStatus::QualifyingSurvivingSpouse => "QualifyingSurvivingSpouse",
            FilingStatus::HeadOfHousehold => "HeadOfHousehold",
        }
    }

    fn calculate_from_brackets(
        brackets: &[SocialSecurityBracket],
        income: f64,
        social_security_income: f64,
    ) -> f64 {
        for bracket in brackets {
            if let Some(upper_bound) = bracket.upper_bound {
                if income <= upper_bound {
                    let deduction = bracket.deduction.unwrap_or(0.0);
                    let taxable_base = (social_security_income - deduction).max(0.0);
                    return taxable_base * bracket.rate;
                }
            } else {
                // No upper bound means this is the highest bracket
                let deduction = bracket.deduction.unwrap_or(0.0);
                let taxable_base = (social_security_income - deduction).max(0.0);
                return taxable_base * bracket.rate;
            }
        }
        0.0
    }

    fn get_federal_rate(&self, year: i32, filing_status: &FilingStatus, income: f64) -> f64 {
        // Get the most recent year if exact year not available
        let mut available_years: Vec<i32> = self.federal_data.keys().cloned().collect();
        if available_years.is_empty() {
            return 0.85; // Default fallback
        }
        available_years.sort_unstable();
        
        let chosen_year = if self.federal_data.contains_key(&year) {
            year
        } else {
            *available_years.last().unwrap()
        };

        let Some(year_data) = self.federal_data.get(&chosen_year) else {
            return 0.85; // Default fallback
        };

        let status_key = Self::status_key(filing_status);
        let Some(brackets) = year_data.get(status_key) else {
            // Fallback to Single if filing status not found
            let Some(brackets) = year_data.get("Single") else {
                return 0.85; // Default fallback
            };
            return Self::calculate_rate_from_brackets(brackets, income);
        };

        Self::calculate_rate_from_brackets(brackets, income)
    }

    fn calculate_rate_from_brackets(brackets: &[SocialSecurityBracket], income: f64) -> f64 {
        for bracket in brackets {
            if let Some(upper_bound) = bracket.upper_bound {
                if income <= upper_bound {
                    return bracket.rate;
                }
            } else {
                // No upper bound means this is the highest bracket
                return bracket.rate;
            }
        }
        0.85 // Default fallback
    }

    /// Calculate taxable social security for a given state/year/age/filing status combination
    pub fn get_taxable_social_security(
        &self,
        state: &str,
        year: i32,
        age: i32,
        filing_status: &FilingStatus,
        income: f64,
        social_security_income: f64,
    ) -> f64 {
        let state_key = state.to_uppercase();
        
        // Handle federal calculation (US) specially
        if state_key == "US" || state_key == "FEDERAL" {
            let federal_rate = self.get_federal_rate(year, filing_status, income);
            return social_security_income * federal_rate;
        }
        
        // Check if this is a state we handle
        let Some(state_years) = self.state_data.get(&state_key) else {
            eprintln!("Error: State '{}' is not currently supported for social security taxation or may not tax social security.", state);
            return 0.0;
        };

        // Find the appropriate year, fallback to latest if not available
        let mut available_years: Vec<i32> = state_years.keys().cloned().collect();
        if available_years.is_empty() {
            eprintln!("No data available for state '{}'", state);
            return 0.0;
        }
        available_years.sort_unstable();

        let chosen_year = if state_years.contains_key(&year) {
            year
        } else {
            let latest = *available_years.last().unwrap();
            eprintln!("Year {} not supported for {}. Defaulting to latest ({}).", year, state, latest);
            latest
        };

        let Some(year_data) = state_years.get(&chosen_year) else {
            return 0.0;
        };

        match year_data {
            StateYearDataEnum::ByFilingStatus(filing_status_map) => {
                // State has brackets organized by filing status only
                let status_key = Self::status_key(filing_status);
                let filing_status_value = if let Some(value) = filing_status_map.get(status_key) {
                    value
                } else if let Some(value) = filing_status_map.get("Single") {
                    value
                } else {
                    // Use federal rules as fallback
                    let federal_rate = self.get_federal_rate(year, filing_status, income);
                    return social_security_income * federal_rate;
                };
                
                self.calculate_from_filing_status_value(filing_status_value, year, filing_status, income, social_security_income)
            }
            StateYearDataEnum::ByAge(age_map) => {
                // State has brackets organized by age ranges
                let age_key = self.determine_age_key(age, age_map);
                let Some(age_data) = age_map.get(&age_key) else {
                    // If no exact age match, try to find the closest or default
                    let default_age_key = age_map.keys().next().cloned().unwrap_or_default();
                    let Some(age_data) = age_map.get(&default_age_key) else {
                        return 0.0;
                    };
                    return self.get_from_age_data_wrapper(age_data, filing_status, income, social_security_income);
                };
                self.get_from_age_data_wrapper(age_data, filing_status, income, social_security_income)
            }
        }
    }

    fn calculate_from_filing_status_value(
        &self,
        filing_status_value: &FilingStatusValue,
        year: i32,
        filing_status: &FilingStatus,
        income: f64,
        social_security_income: f64,
    ) -> f64 {
        match filing_status_value {
            FilingStatusValue::Brackets(brackets) => {
                Self::calculate_from_brackets(brackets, income, social_security_income)
            }
            FilingStatusValue::Federal(_) => {
                // State uses federal rules
                let federal_rate = self.get_federal_rate(year, filing_status, income);
                social_security_income * federal_rate
            }
        }
    }

    fn determine_age_key(&self, age: i32, age_map: &HashMap<String, AgeData>) -> String {
        // Convert age keys to integers and find the appropriate one
        let mut age_keys: Vec<i32> = age_map.keys()
            .filter_map(|k| k.parse::<i32>().ok())
            .collect();
        age_keys.sort_unstable();

        // Find the highest age key that is <= the given age
        for &age_threshold in age_keys.iter().rev() {
            if age >= age_threshold {
                return age_threshold.to_string();
            }
        }

        // If no match found, use the lowest age key
        age_keys.first().map(|&a| a.to_string()).unwrap_or_default()
    }

    fn get_from_age_data_wrapper(
        &self,
        age_data: &AgeData,
        filing_status: &FilingStatus,
        income: f64,
        social_security_income: f64,
    ) -> f64 {
        match age_data {
            AgeData::Direct(filing_status_map) => {
                self.get_from_filing_status_map(filing_status_map, filing_status, income, social_security_income)
            }
            AgeData::Wrapped(vec_of_maps) => {
                // Take the first map from the array (there's usually only one)
                if let Some(filing_status_map) = vec_of_maps.first() {
                    self.get_from_filing_status_map(filing_status_map, filing_status, income, social_security_income)
                } else {
                    // Use federal rules as fallback
                    let federal_rate = self.get_federal_rate(2024, filing_status, income);
                    social_security_income * federal_rate
                }
            }
        }
    }

    fn get_from_filing_status_map(
        &self,
        filing_status_map: &HashMap<String, FilingStatusValue>,
        filing_status: &FilingStatus,
        income: f64,
        social_security_income: f64,
    ) -> f64 {
        let status_key = Self::status_key(filing_status);
        let filing_status_value = if let Some(value) = filing_status_map.get(status_key) {
            value
        } else if let Some(value) = filing_status_map.get("Single") {
            value
        } else {
            // Use federal rules as fallback
            let federal_rate = self.get_federal_rate(2024, filing_status, income);
            return social_security_income * federal_rate;
        };
        
        self.calculate_from_filing_status_value(filing_status_value, 2024, filing_status, income, social_security_income)
    }
} 
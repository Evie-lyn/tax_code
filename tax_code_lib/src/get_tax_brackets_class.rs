use std::collections::HashMap;
use serde::Deserialize;

use crate::FilingStatus;
use crate::brackets::Bracket;

const TAX_BRACKETS_JSON: &str = include_str!("states_brackets.json");
const YEARLY_LIMIT_INCREASE_RATE: f64 = 0.02; // 2.0%

#[derive(Debug, Deserialize, Clone)]
struct JsonBracket {
    rate: f64,
    upper_bound: Option<f64>,
}

type StateYearStatusBrackets = HashMap<String, HashMap<String, HashMap<String, Vec<JsonBracket>>>>;

pub struct StateIncomeTaxBrackets {
    // state -> year -> status -> brackets
    data: HashMap<String, HashMap<i32, HashMap<String, Vec<JsonBracket>>>>,
}

impl StateIncomeTaxBrackets {
    pub fn load() -> Result<Self, String> {
        let parsed: StateYearStatusBrackets = serde_json::from_str(TAX_BRACKETS_JSON)
            .map_err(|e| format!("Failed to parse states_brackets.json: {}", e))?;

        // Normalize years to i32 for easier math and store by state uppercase
        let mut data: HashMap<String, HashMap<i32, HashMap<String, Vec<JsonBracket>>>> = HashMap::new();
        for (state, years_map) in parsed.into_iter() {
            let mut inner: HashMap<i32, HashMap<String, Vec<JsonBracket>>> = HashMap::new();
            for (year_str, status_map) in years_map.into_iter() {
                if let Ok(year) = year_str.parse::<i32>() {
                    inner.insert(year, status_map);
                }
            }
            data.insert(state.to_uppercase(), inner);
        }

        Ok(Self { data })
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

    /// Get brackets for the given state/year/status.
    /// - If the exact year is missing, uses the most recent available year for that state and
    ///   increases the upper_bound limits by 1.02% per missing year (rates unchanged).
    /// - If the filing status is missing for the chosen year, returns an error.
    pub fn get(&self, state: &str, year: i32, filing_status: &FilingStatus) -> Result<Vec<Bracket>, String> {
        let state_key = state.to_uppercase();
        let Some(years_map) = self.data.get(&state_key) else {
            return Err(format!("State '{}' is not supported", state));
        };

        // Determine chosen year and years missing
        let mut available_years: Vec<i32> = years_map.keys().cloned().collect();
        if available_years.is_empty() {
            return Err(format!("No bracket data available for state '{}'", state));
        }
        available_years.sort_unstable();
        let latest_year = *available_years.last().unwrap();

        let (chosen_year, years_missing) = if years_map.contains_key(&year) {
            (year, 0)
        } else {
            // Fallback to most recent year. Only scale forward when requesting a later year.
            let missing = if year > latest_year { year - latest_year } else { 0 };
            (latest_year, missing)
        };

        let Some(status_map) = years_map.get(&chosen_year) else {
            return Err(format!("Internal error: year '{}' not found after selection for state '{}'", chosen_year, state));
        };

        let status_key = Self::status_key(filing_status);
        let Some(json_brackets) = status_map.get(status_key) else {
            return Err(format!(
                "Filing status '{}' missing for state '{}' in year {}",
                status_key, state, chosen_year
            ));
        };

        // Scale upper bounds if needed
        let factor = (1.0 + YEARLY_LIMIT_INCREASE_RATE).powi(years_missing);
        let out: Vec<Bracket> = json_brackets
            .iter()
            .map(|b| {
                let ub = match b.upper_bound {
                    Some(v) => Some(v * factor),
                    None => None,
                };
                Bracket(ub.unwrap_or(f64::INFINITY), b.rate)
            })
            .collect();

        Ok(out)
    }
} 
use std::collections::HashMap;

use crate::FilingStatus;
use crate::Deduction;

const DEDUCTIONS_JSON: &str = include_str!("states_deductions.json");
const YEARLY_LIMIT_INCREASE_RATE: f64 = 0.02; // 2.0%

type StateYearStatusDeductions = HashMap<String, HashMap<String, HashMap<String, f64>>>;

/// Runtime-loaded state deductions calculator
///
/// This struct loads state tax deduction data from JSON files at runtime rather than
/// generating code at build time. It provides deduction amounts for different states,
/// years, and filing statuses.
///
/// # Features
/// - Loads deduction data from `states_deductions.json`
/// - Handles income-based deductions for special states (AL, WI) by delegating to `income_based_deduction` module
/// - Automatically scales deductions for future years using a 2% annual increase rate
/// - Case-insensitive state lookup
/// - Falls back to most recent year's data when requested year is unavailable
///
/// # Example
/// ```
/// use tax_code::{get_deductions_class::StateDeductions, FilingStatus};
///
/// let deductions = StateDeductions::load().expect("Failed to load deductions");
/// let deduction = deductions.get("CA", 2024, &FilingStatus::Single, 50000.0);
/// assert_eq!(deduction.standard_deduction, 5540.0);
/// ```
pub struct StateDeductions {
    // state -> year -> status -> deduction amount
    data: HashMap<String, HashMap<i32, HashMap<String, f64>>>,
}

impl StateDeductions {
    pub fn load() -> Result<Self, String> {
        let parsed: StateYearStatusDeductions = serde_json::from_str(DEDUCTIONS_JSON)
            .map_err(|e| format!("Failed to parse states_deductions.json: {}", e))?;

        // Normalize years to i32 for easier math and store by state uppercase
        let mut data: HashMap<String, HashMap<i32, HashMap<String, f64>>> = HashMap::new();
        for (state, years_map) in parsed.into_iter() {
            let mut inner: HashMap<i32, HashMap<String, f64>> = HashMap::new();
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

    /// Get deductions for the given state/year/status/income.
    /// - For states with income-based deductions (AL, WI), delegates to income_based_deduction module
    /// - For other states, returns standard deduction from JSON
    /// - If the exact year is missing, uses the most recent available year for that state and
    ///   increases the deduction by 2% per missing year.
    /// - If the filing status is missing for the chosen year, returns an error with 0.0 deduction.
    pub fn get(&self, state: &str, year: i32, filing_status: &FilingStatus, income: f64) -> Deduction {
        let state_key = state.to_uppercase();
        
        // Handle income-based deductions for AL and WI
        match state_key.as_str() {
            "AL" => {
                // Alabama has income-based deductions
                match year {
                    2024 => return crate::income_based_deduction::al_standard_deduction_2024(income, filing_status),
                    _ => {
                        eprintln!("Year {} not supported for AL. Defaulting to 2024 deduction.", year);
                        return crate::income_based_deduction::al_standard_deduction_2024(income, filing_status);
                    }
                }
            },
            "WI" => {
                // Wisconsin has income-based deductions
                match year {
                    2024 => return crate::income_based_deduction::wi_standard_deduction_2024(income, filing_status),
                    _ => {
                        eprintln!("Year {} not supported for WI. Defaulting to 2024 deduction.", year);
                        return crate::income_based_deduction::wi_standard_deduction_2024(income, filing_status);
                    }
                }
            },
            _ => {}
        }

        // Standard deductions for all other states
        let Some(years_map) = self.data.get(&state_key) else {
            eprintln!("Error: State '{}' is not currently supported for deductions. Defaulting to 0 deduction.", state);
            return Deduction { standard_deduction: 0.0 };
        };

        // Determine chosen year and years missing
        let mut available_years: Vec<i32> = years_map.keys().cloned().collect();
        if available_years.is_empty() {
            eprintln!("No deduction data available for state '{}'. Defaulting to 0 deduction.", state);
            return Deduction { standard_deduction: 0.0 };
        }
        available_years.sort_unstable();
        let latest_year = *available_years.last().unwrap();

        let (chosen_year, years_missing) = if years_map.contains_key(&year) {
            (year, 0)
        } else {
            // Fallback to most recent year. Only scale forward when requesting a later year.
            eprintln!("Year {} not supported for {}. Defaulting to {} deduction.", year, state, latest_year);
            let missing = if year > latest_year { year - latest_year } else { 0 };
            (latest_year, missing)
        };

        let Some(status_map) = years_map.get(&chosen_year) else {
            eprintln!("Internal error: year '{}' not found after selection for state '{}'. Defaulting to 0 deduction.", chosen_year, state);
            return Deduction { standard_deduction: 0.0 };
        };

        let status_key = Self::status_key(filing_status);
        let Some(&deduction_amount) = status_map.get(status_key) else {
            eprintln!(
                "Filing status '{}' missing for state '{}' in year {}. Defaulting to 0 deduction.",
                status_key, state, chosen_year
            );
            return Deduction { standard_deduction: 0.0 };
        };

        // Scale deduction if needed (for future years)
        let factor = (1.0 + YEARLY_LIMIT_INCREASE_RATE).powi(years_missing);
        let scaled_deduction = deduction_amount * factor;

        Deduction { standard_deduction: scaled_deduction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_state_deductions() {
        let deductions = StateDeductions::load();
        assert!(deductions.is_ok(), "Failed to load state deductions");
    }

    #[test]
    fn test_get_california_deduction() {
        let deductions = StateDeductions::load().expect("Failed to load deductions");
        let result = deductions.get("CA", 2024, &FilingStatus::Single, 50000.0);
        assert_eq!(result.standard_deduction, 5540.0, "CA Single 2024 should be 5540");
    }

    #[test]
    fn test_get_alabama_income_based_deduction() {
        let deductions = StateDeductions::load().expect("Failed to load deductions");
        // Alabama has income-based deductions
        let result = deductions.get("AL", 2024, &FilingStatus::Single, 20000.0);
        // Low income should get max deduction
        assert_eq!(result.standard_deduction, 3000.0, "AL Single 2024 low income should be 3000");
    }

    #[test]
    fn test_get_wisconsin_income_based_deduction() {
        let deductions = StateDeductions::load().expect("Failed to load deductions");
        // Wisconsin has income-based deductions
        let result = deductions.get("WI", 2024, &FilingStatus::Single, 20000.0);
        // Should use income_based_deduction module
        assert!(result.standard_deduction > 0.0, "WI deduction should be positive");
    }

    #[test]
    fn test_unsupported_state() {
        let deductions = StateDeductions::load().expect("Failed to load deductions");
        let result = deductions.get("XX", 2024, &FilingStatus::Single, 50000.0);
        assert_eq!(result.standard_deduction, 0.0, "Unsupported state should return 0");
    }

    #[test]
    fn test_future_year_scaling() {
        let deductions = StateDeductions::load().expect("Failed to load deductions");
        // Request a future year (should scale from latest available year)
        let result_2024 = deductions.get("CA", 2024, &FilingStatus::Single, 50000.0);
        let result_2026 = deductions.get("CA", 2026, &FilingStatus::Single, 50000.0);
        
        // 2026 should be scaled up by 2% per year (2 years)
        let expected_2026 = result_2024.standard_deduction * 1.02 * 1.02;
        assert!(
            (result_2026.standard_deduction - expected_2026).abs() < 0.01,
            "Future year should be scaled: expected {}, got {}",
            expected_2026,
            result_2026.standard_deduction
        );
    }

    #[test]
    fn test_case_insensitive_state() {
        let deductions = StateDeductions::load().expect("Failed to load deductions");
        let result_upper = deductions.get("CA", 2024, &FilingStatus::Single, 50000.0);
        let result_lower = deductions.get("ca", 2024, &FilingStatus::Single, 50000.0);
        assert_eq!(
            result_upper.standard_deduction,
            result_lower.standard_deduction,
            "State lookup should be case insensitive"
        );
    }
}


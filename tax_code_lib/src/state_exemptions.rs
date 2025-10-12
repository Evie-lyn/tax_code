use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::FilingStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearExemptions {
    #[serde(rename = "Single")]
    pub single: f64,
    #[serde(rename = "MarriedFilingSeparately")]
    pub married_filing_separately: f64,
    #[serde(rename = "MarriedFilingJointly")]
    pub married_filing_jointly: f64,
    #[serde(rename = "QualifyingSurvivingSpouse")]
    pub qualifying_surviving_spouse: f64,
    #[serde(rename = "HeadOfHousehold")]
    pub head_of_household: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateExemptions {
    #[serde(flatten)]
    pub years: HashMap<String, YearExemptions>,
}

/// Struct to encapsulate state exemption calculation results
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExemptionAmount {
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct StateExemptionsCalculator {
    exemptions: HashMap<String, StateExemptions>,
}

impl StateExemptionsCalculator {
    /// Load state exemptions from JSON file
    pub fn load() -> Result<Self, String> {
        let exemptions_str = include_str!("state_exemptions.json");
        let exemptions: HashMap<String, StateExemptions> = serde_json::from_str(exemptions_str)
            .map_err(|e| format!("Failed to parse state exemptions JSON: {}", e))?;
        
        Ok(Self { exemptions })
    }

    /// Calculate state exemptions for a given state, year, and filing status
    /// This method first checks for function-based exemptions, then falls back to JSON
    pub fn calc_state_exemptions(
        &self,
        state: &str,
        year: i32,
        filing_status: &FilingStatus,
    ) -> f64 {
        let state_lower = state.to_lowercase();
        
        // Try function-based exemptions first
        if let Some(exemption) = self.get_function_based_exemption(&state_lower, year, filing_status) {
            return exemption;
        }
        
        // Fall back to JSON-based exemptions
        self.get_exemption(state, year, filing_status)
    }

    /// Get exemption amount for a specific state, year, and filing status from JSON
    pub fn get_exemption(&self, state: &str, year: i32, filing_status: &FilingStatus) -> f64 {
        let state_upper = state.to_uppercase();
        let state_exemptions = match self.exemptions.get(&state_upper) {
            Some(exemptions) => exemptions,
            None => return 0.0, // No exemptions defined for this state
        };

        let year_str = year.to_string();
        let year_exemptions = match state_exemptions.years.get(&year_str) {
            Some(exemptions) => exemptions,
            None => return 0.0, // No exemptions defined for this year
        };

        match filing_status {
            FilingStatus::Single => year_exemptions.single,
            FilingStatus::MarriedFilingSeparately => year_exemptions.married_filing_separately,
            FilingStatus::MarriedFilingJointly => year_exemptions.married_filing_jointly,
            FilingStatus::QualifyingSurvivingSpouse => year_exemptions.qualifying_surviving_spouse,
            FilingStatus::HeadOfHousehold => year_exemptions.head_of_household,
        }
    }

    /// Get function-based exemption for states that have custom logic
    fn get_function_based_exemption(
        &self,
        state: &str,
        year: i32,
        filing_status: &FilingStatus,
    ) -> Option<f64> {
        match (state, year) {
            ("me" | "maine", 2024) => Some(maine_exemptions_2024(filing_status)),
            // Add more state/year combinations here as needed
            _ => None,
        }
    }

    /// Check if a state has exemptions defined
    pub fn has_exemptions(&self, state: &str) -> bool {
        let state_upper = state.to_uppercase();
        self.exemptions.contains_key(&state_upper)
    }

    /// Get all states that have exemptions defined
    pub fn get_states_with_exemptions(&self) -> Vec<&String> {
        self.exemptions.keys().collect()
    }
}

/// Maine personal exemptions for 2024
/// Personal exemption: $5,150 per individual
fn maine_exemptions_2024(filing_status: &FilingStatus) -> f64 {
    match filing_status {
        FilingStatus::Single => 5150.0,
        FilingStatus::MarriedFilingSeparately => 5150.0,
        FilingStatus::MarriedFilingJointly => 10300.0, // $5,150 x 2
        FilingStatus::QualifyingSurvivingSpouse => 10300.0, // $5,150 x 2
        FilingStatus::HeadOfHousehold => 5150.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_exemptions() {
        let calculator = StateExemptionsCalculator::load().unwrap();
        assert!(calculator.has_exemptions("MS"));
    }

    #[test]
    fn test_get_exemption() {
        let calculator = StateExemptionsCalculator::load().unwrap();
        
        // Test Mississippi exemptions for 2024 (from JSON)
        assert_eq!(calculator.get_exemption("MS", 2024, &FilingStatus::Single), 6000.0);
        assert_eq!(calculator.get_exemption("MS", 2024, &FilingStatus::MarriedFilingJointly), 12000.0);
        assert_eq!(calculator.get_exemption("MS", 2024, &FilingStatus::HeadOfHousehold), 8000.0);
        
        // Test non-existent state
        assert_eq!(calculator.get_exemption("XX", 2024, &FilingStatus::Single), 0.0);
        
        // Test non-existent year
        assert_eq!(calculator.get_exemption("MS", 2020, &FilingStatus::Single), 0.0);
    }

    #[test]
    fn test_maine_exemptions_2024() {
        let calculator = StateExemptionsCalculator::load().unwrap();
        
        // Test Maine exemptions for 2024 (function-based)
        assert_eq!(calculator.calc_state_exemptions("ME", 2024, &FilingStatus::Single), 5150.0);
        assert_eq!(calculator.calc_state_exemptions("ME", 2024, &FilingStatus::MarriedFilingJointly), 10300.0);
        assert_eq!(calculator.calc_state_exemptions("ME", 2024, &FilingStatus::MarriedFilingSeparately), 5150.0);
        assert_eq!(calculator.calc_state_exemptions("ME", 2024, &FilingStatus::HeadOfHousehold), 5150.0);
        assert_eq!(calculator.calc_state_exemptions("ME", 2024, &FilingStatus::QualifyingSurvivingSpouse), 10300.0);
        
        // Test with lowercase state abbreviation
        assert_eq!(calculator.calc_state_exemptions("me", 2024, &FilingStatus::Single), 5150.0);
        
        // Test with full state name
        assert_eq!(calculator.calc_state_exemptions("maine", 2024, &FilingStatus::Single), 5150.0);
    }

    #[test]
    fn test_calc_state_exemptions_fallback_to_json() {
        let calculator = StateExemptionsCalculator::load().unwrap();
        
        // Test that calc_state_exemptions falls back to JSON for states without function-based exemptions
        assert_eq!(calculator.calc_state_exemptions("MS", 2024, &FilingStatus::Single), 6000.0);
        assert_eq!(calculator.calc_state_exemptions("MS", 2024, &FilingStatus::MarriedFilingJointly), 12000.0);
    }

    #[test]
    fn test_calc_state_exemptions_nonexistent_state() {
        let calculator = StateExemptionsCalculator::load().unwrap();
        
        // Test non-existent state returns 0.0
        assert_eq!(calculator.calc_state_exemptions("XX", 2024, &FilingStatus::Single), 0.0);
    }
} 
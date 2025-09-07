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

    /// Get exemption amount for a specific state, year, and filing status
    pub fn get_exemption(&self, state: &str, year: i32, filing_status: &FilingStatus) -> f64 {
        let state_exemptions = match self.exemptions.get(state) {
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

    /// Check if a state has exemptions defined
    pub fn has_exemptions(&self, state: &str) -> bool {
        self.exemptions.contains_key(state)
    }

    /// Get all states that have exemptions defined
    pub fn get_states_with_exemptions(&self) -> Vec<&String> {
        self.exemptions.keys().collect()
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
        
        // Test Mississippi exemptions for 2024
        assert_eq!(calculator.get_exemption("MS", 2024, &FilingStatus::Single), 6000.0);
        assert_eq!(calculator.get_exemption("MS", 2024, &FilingStatus::MarriedFilingJointly), 12000.0);
        assert_eq!(calculator.get_exemption("MS", 2024, &FilingStatus::HeadOfHousehold), 8000.0);
        
        // Test non-existent state
        assert_eq!(calculator.get_exemption("XX", 2024, &FilingStatus::Single), 0.0);
        
        // Test non-existent year
        assert_eq!(calculator.get_exemption("MS", 2020, &FilingStatus::Single), 0.0);
    }
} 
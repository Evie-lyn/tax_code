use crate::FilingStatus;
use crate::Deduction;
use serde::{Deserialize, Serialize}; 
use std::collections::HashMap; 
use std::sync::OnceLock; 

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StepBracket {
    pub bracket_start: f64,
    pub bracket_end: f64,
    pub bracket_size: f64,
    pub deduction_start: f64,
    pub deduction_end: f64,
    pub fixed_deduction_below: f64,
    pub fixed_deduction_above: f64,
}

#[derive(Debug, Deserialize)]
struct StepDeductionData {
    #[serde(flatten)] 
    states: HashMap<String, HashMap<String, HashMap<String, StepBracket>>>,
}

/// A reusable class for calculating income-based deductions using step brackets
/// The data is loaded once from states_step_deduction.json and cached for reuse
pub struct StepDeductionCalculator {
    data: &'static StepDeductionData,
}

static STEP_DEDUCTION_DATA: OnceLock<StepDeductionData> = OnceLock::new();

impl StepDeductionCalculator {
    /// Creates a new StepDeductionCalculator instance
    /// The JSON data is loaded once and cached globally
    pub fn new() -> Self {
        let data = STEP_DEDUCTION_DATA.get_or_init(|| {
            let content = include_str!("states_step_deduction.json");
            serde_json::from_str(content)
                .expect("Failed to parse JSON from states_step_deduction.json. Check its format.")
        });
        
        Self { data }
    }

    /// Gets the step bracket for a specific state, year, and filing status
    pub fn get_step_bracket(&self, state: &str, year: &str, filing_status: &FilingStatus) -> Option<&StepBracket> {
        let filing_status_str = self.filing_status_to_string(filing_status);
        
        self.data.states
            .get(state)?
            .get(year)?
            .get(filing_status_str)
    }

    /// Converts FilingStatus enum to the string representation used in JSON
    fn filing_status_to_string(&self, filing_status: &FilingStatus) -> &str {
        match filing_status {
            FilingStatus::Single => "Single",
            FilingStatus::MarriedFilingSeparately => "MarriedFilingSeparately",
            FilingStatus::MarriedFilingJointly => "MarriedFilingJointly",
            FilingStatus::QualifyingSurvivingSpouse => "QualifyingSurvivingSpouse",
            FilingStatus::HeadOfHousehold => "HeadOfHousehold",
        }
    }

    /// Calculates the deduction based on income and step bracket
    pub fn calculate_step_deduction(&self, income: f64, step_bracket: &StepBracket) -> f64 {
        calculate_step_deduction(income, step_bracket)
    }

    /// Generic method to calculate standard deduction for any state/year
    /// Returns None if the state/year combination is not found
    pub fn calculate_deduction(&self, state: &str, year: &str, income: f64, filing_status: &FilingStatus) -> Option<Deduction> {
        let step_bracket = self.get_step_bracket(state, year, filing_status)?;
        
        // Check if income is below the step bracket range
        if income < step_bracket.bracket_start {
            return Some(Deduction { standard_deduction: step_bracket.fixed_deduction_below });
        }
        
        // Check if income is above the step bracket range
        if income > step_bracket.bracket_end {
            return Some(Deduction { standard_deduction: step_bracket.fixed_deduction_above });
        }
        
        // Income is within the step bracket range - calculate stepped deduction
        Some(Deduction { standard_deduction: self.calculate_step_deduction(income, step_bracket) })
    }
}

impl Default for StepDeductionCalculator {
    fn default() -> Self {
        Self::new()
    }
}


pub fn calculate_step_deduction(income: f64, step_bracket: &StepBracket) -> f64 {
    let num_lines = ((step_bracket.bracket_end - step_bracket.bracket_start + 1.0) / step_bracket.bracket_size).ceil();
    let num_lines = num_lines.max(1.0);

    let num_decrements = (num_lines - 1.0).max(0.0);

    let decrease_per_step = if num_decrements > 0.0 {
        (step_bracket.deduction_start - step_bracket.deduction_end) / num_decrements
    } else {
        0.0
    };

    let current_line_index = ((income - step_bracket.bracket_start).max(0.0) / step_bracket.bracket_size).floor();

    let calculated_deduction = step_bracket.deduction_start - (current_line_index * decrease_per_step);

    calculated_deduction.max(step_bracket.deduction_end).min(step_bracket.deduction_start)
}

//Following Alabama 2024 Standard Deduction Chart
pub fn al_standard_deduction_2024(income: f64, filing_status: &FilingStatus) -> Deduction {
    let calculator = StepDeductionCalculator::new();
    
    if let Some(step_bracket) = calculator.get_step_bracket("AL", "2024", filing_status) {
        // Use bracket bounds and fixed deduction values from JSON
        if income < step_bracket.bracket_start {
            // Below the step bracket range - use fixed deduction from JSON
            Deduction { standard_deduction: step_bracket.fixed_deduction_below }
        } else if income > step_bracket.bracket_end {
            // Above the step bracket range - use fixed deduction from JSON
            Deduction { standard_deduction: step_bracket.fixed_deduction_above }
        } else {
            // Within the step bracket range - calculate stepped deduction
            Deduction { standard_deduction: calculator.calculate_step_deduction(income, step_bracket) }
        }
    } else {
        eprintln!("Error: StepBracket data not found for filing status in AL 2024.");
        return Deduction { standard_deduction: 0.0 }; 
    }
}

//Following Wisconsin 2024 Standard Deduction Chart
pub fn wi_standard_deduction_2024(income: f64, filing_status: &FilingStatus) -> Deduction {
    let calculator = StepDeductionCalculator::new();
    
    if let Some(step_bracket) = calculator.get_step_bracket("WI", "2024", filing_status) {
        // Use bracket bounds and fixed deduction values from JSON
        if income < step_bracket.bracket_start {
            // Below the step bracket range - use fixed deduction from JSON
            Deduction { standard_deduction: step_bracket.fixed_deduction_below }
        } else if income > step_bracket.bracket_end {
            // Above the step bracket range - use fixed deduction from JSON
            Deduction { standard_deduction: step_bracket.fixed_deduction_above }
        } else {
            // Within the step bracket range - calculate stepped deduction
            Deduction { standard_deduction: calculator.calculate_step_deduction(income, step_bracket) }
        }
    } else {
        eprintln!("Error: StepBracket data not found for filing status in WI 2024.");
        return Deduction { standard_deduction: 0.0 }; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FilingStatus; 
//Alabama
    #[test]
    fn test_married_filing_joint_deduction() {
        let income = 26500.0; 
        let filing_status = FilingStatus::MarriedFilingJointly;
        let expected_deduction = 8150.0;
        let result = al_standard_deduction_2024(income, &filing_status);
        assert_eq!(result.standard_deduction, expected_deduction, "MFJ deduction mismatch for income {}", income);
    }

    #[test]
    fn test_married_filing_separate_deduction() {
        let income = 16500.0; 
        let filing_status = FilingStatus::MarriedFilingSeparately;
        let expected_deduction = 2930.0;
        let result = al_standard_deduction_2024(income, &filing_status);
        assert_eq!(result.standard_deduction, expected_deduction, "MFS deduction mismatch for income {}", income);
    }

    #[test]
    fn test_head_of_household_deduction() {
        let income = 29000.0; 
        let filing_status = FilingStatus::HeadOfHousehold; 
        let expected_deduction = 4255.0;
        let result = al_standard_deduction_2024(income, &filing_status);
        assert_eq!(result.standard_deduction, expected_deduction, "HOH deduction mismatch for income {}", income);
    }

    #[test]
    fn test_single_deduction() {
        let income = 26500.0; 
        let filing_status = FilingStatus::Single;
        let expected_deduction = 2950.0;
        let result = al_standard_deduction_2024(income, &filing_status);
        assert_eq!(result.standard_deduction, expected_deduction, "Single deduction mismatch for income {}", income);
    }

    #[test]
    fn test_single_fixed_lower_deduction() {
        let income = 25000.0; 
        let filing_status = FilingStatus::Single;
        let expected_deduction = 3000.0;
        let result = al_standard_deduction_2024(income, &filing_status);
        assert_eq!(result.standard_deduction, expected_deduction, "Single fixed lower deduction mismatch");
    }

    #[test]
    fn test_single_fixed_upper_deduction() {
        let income = 36000.0; 
        let filing_status = FilingStatus::Single;
        let expected_deduction = 2500.0;
        let result = al_standard_deduction_2024(income, &filing_status);
        assert_eq!(result.standard_deduction, expected_deduction, "Single fixed upper deduction mismatch");
    }

     //Wisconsin

     #[test]
     fn test_wi_single_fixed_lower_deduction() {
         let income = 12000.0; 
         let filing_status = FilingStatus::Single;
         let expected_deduction = 13230.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI Single fixed lower deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_single_fixed_upper_deduction() {
         let income = 130000.0; 
         let filing_status = FilingStatus::Single;
         let expected_deduction = 0.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI Single fixed upper deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_single_sliding_deduction_start() {
         let income = 13000.0;
         let filing_status = FilingStatus::Single;
         let expected_deduction = 13230.0; 
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI Single sliding deduction start mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_single_sliding_deduction_near_end() {
         let income = 129499.0;
         let filing_status = FilingStatus::Single;
         let expected_deduction = 8.0; 
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert!((result.standard_deduction - expected_deduction).abs() < 0.01, "WI Single sliding deduction near end mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_married_filing_jointly_fixed_lower_deduction() {
         let income = 12000.0;
         let filing_status = FilingStatus::MarriedFilingJointly;
         let expected_deduction = 24490.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI MFJ fixed lower deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_married_filing_jointly_fixed_upper_deduction() {
         let income = 152000.0; 
         let filing_status = FilingStatus::MarriedFilingJointly;
         let expected_deduction = 0.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI MFJ fixed upper deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_married_filing_jointly_sliding_deduction_start() {
         let income = 13000.0; 
         let filing_status = FilingStatus::MarriedFilingJointly;
         let expected_deduction = 24490.0; 
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI MFJ sliding deduction start mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_married_filing_separately_fixed_lower_deduction() {
         let income = 12000.0; 
         let filing_status = FilingStatus::MarriedFilingSeparately;
         let expected_deduction = 11630.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI MFS fixed lower deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_married_filing_separately_fixed_upper_deduction() {
         let income = 72000.0; 
         let filing_status = FilingStatus::MarriedFilingSeparately;
         let expected_deduction = 0.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI MFS fixed upper deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_head_of_household_fixed_lower_deduction() {
         let income = 12000.0; 
         let filing_status = FilingStatus::HeadOfHousehold;
         let expected_deduction = 17090.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI HOH fixed lower deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_head_of_household_fixed_upper_deduction() {
         let income = 130000.0; 
         let filing_status = FilingStatus::HeadOfHousehold;
         let expected_deduction = 0.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI HOH fixed upper deduction mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_head_of_household_sliding_deduction_start() {
         let income = 13000.0; 
         let filing_status = FilingStatus::HeadOfHousehold;
         let expected_deduction = 17090.0; 
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert_eq!(result.standard_deduction, expected_deduction, "WI HOH sliding deduction start mismatch for income {}", income);
     }
 
     #[test]
     fn test_wi_head_of_household_sliding_deduction_near_end() {
         let income = 129499.0; 
         let filing_status = FilingStatus::HeadOfHousehold;
         let expected_deduction = 8.0;
         let result = wi_standard_deduction_2024(income, &filing_status);
         assert!((result.standard_deduction - expected_deduction).abs() < 0.01, "WI HOH sliding deduction near end mismatch for income {}", income);
     }
}
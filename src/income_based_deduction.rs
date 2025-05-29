use crate::FilingStatus;
use crate::Deduction;
use serde::{Deserialize, Serialize}; 
use std::collections::HashMap; 
use std::fs; 
use std::sync::OnceLock; 

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StepBracket {
    pub bracket_start: f64,
    pub bracket_end: f64,
    pub bracket_size: f64,
    pub deduction_start: f64,
    pub deduction_end: f64,
}

#[derive(Debug, Deserialize)]
struct StepDeductionData {
    #[serde(flatten)] 
    states: HashMap<String, HashMap<String, HashMap<String, StepBracket>>>,
}

static STEP_DEDUCTION_DATA: OnceLock<StepDeductionData> = OnceLock::new();

fn load_step_deduction_data() -> &'static StepDeductionData {
    STEP_DEDUCTION_DATA.get_or_init(|| {
        let path = "src/step_deduction.json"; 
        let content = fs::read_to_string(path)
            .expect(&format!("Failed to read {}. Make sure it's in the project root or correct path.", path));
        serde_json::from_str(&content)
            .expect(&format!("Failed to parse JSON from {}. Check its format.", path))
    })
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
    let data = load_step_deduction_data();

    let filing_status_str = match filing_status {
        FilingStatus::Single => "Single",
        FilingStatus::MarriedFilingSeparately => "MarriedFilingSeparately",
        FilingStatus::MarriedFilingJointly => "MarriedFilingJointly",
        FilingStatus::QualifyingSurvivingSpouse => "QualifyingSurvivingSpouse",
        FilingStatus::HeadOfHousehold => "HeadOfHousehold",
    };

    // For Alabama Standard Deduction
    if let Some(state_data) = data.states.get("AL") {
        if let Some(year_data) = state_data.get("2024") {
            if let Some(step_bracket) = year_data.get(filing_status_str) {

                return match filing_status { 
                    FilingStatus::Single => {
                        if income <= 25999.0 {
                            Deduction { standard_deduction: 3000.0 }
                        } else if income >= 35500.0 {
                            Deduction { standard_deduction: 2500.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                    FilingStatus::MarriedFilingSeparately => {
                        if income <= 12999.0 {
                            Deduction { standard_deduction: 4250.0 }
                        } else if income >= 17750.0 {
                            Deduction { standard_deduction: 2500.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                    FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => {
                        if income <= 25999.0 {
                            Deduction { standard_deduction: 8500.0 }
                        } else if income >= 35500.0 {
                            Deduction { standard_deduction: 5000.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                    FilingStatus::HeadOfHousehold => {
                        if income <= 25999.0 {
                            Deduction { standard_deduction: 5200.0 }
                        } else if income >= 35500.0 {
                            Deduction { standard_deduction: 2500.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                }; 
            } else {
                eprintln!("Error: StepBracket data not found for filing status {} in AL 2024.", filing_status_str);
                return Deduction { standard_deduction: 0.0 }; 
            }
        } else {
            eprintln!("Error: Year 2024 data not found for AL in step_deduction.json.");
            return Deduction { standard_deduction: 0.0 }; 
        }
    } else {
        eprintln!("Error: State AL data not found in step_deduction.json.");
        return Deduction { standard_deduction: 0.0 }; 
    }
}

//Following Wisconsin 2024 Standard Deduction Chart
pub fn wi_standard_deduction_2024(income: f64, filing_status: &FilingStatus) -> Deduction {
    let data = load_step_deduction_data();

    let filing_status_str = match filing_status {
        FilingStatus::Single => "Single",
        FilingStatus::MarriedFilingSeparately => "MarriedFilingSeparately",
        FilingStatus::MarriedFilingJointly => "MarriedFilingJointly",
        FilingStatus::QualifyingSurvivingSpouse => "QualifyingSurvivingSpouse",
        FilingStatus::HeadOfHousehold => "HeadOfHousehold",
    };

    // For Wisconsin Standard Deduction
    if let Some(state_data) = data.states.get("WI") {
        if let Some(year_data) = state_data.get("2024") {
            if let Some(step_bracket) = year_data.get(filing_status_str) {

                return match filing_status { 
                    FilingStatus::Single => {
                        if income <= 18999.0 {
                            Deduction { standard_deduction: 13230.0 }
                        } else if income >= 129500.0 {
                            Deduction { standard_deduction: 0.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                    FilingStatus::MarriedFilingSeparately => {
                        if income <= 12999.0 {
                            Deduction { standard_deduction: 11630.0 }
                        } else if income >= 72000.0 {
                            Deduction { standard_deduction: 0.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                    FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => {
                        if income <= 27499.0 {
                            Deduction { standard_deduction: 24490.0 }
                        } else if income >= 151344.0 {
                            Deduction { standard_deduction: 0.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                    FilingStatus::HeadOfHousehold => {
                        if income <= 18999.0 {
                            Deduction { standard_deduction: 17090.0 }
                        } else if income >= 129500.0 {
                            Deduction { standard_deduction: 0.0 }
                        } else {
                            Deduction { standard_deduction: calculate_step_deduction(income, step_bracket) }
                        }
                    },
                }; 
            } else {
                eprintln!("Error: StepBracket data not found for filing status {} in WI 2024.", filing_status_str);
                return Deduction { standard_deduction: 0.0 }; 
            }
        } else {
            eprintln!("Error: Year 2024 data not found for WI in step_deduction.json.");
            return Deduction { standard_deduction: 0.0 }; 
        }
    } else {
        eprintln!("Error: State WI data not found in step_deduction.json.");
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
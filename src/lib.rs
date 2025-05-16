#[macro_use]
mod brackets;
mod tax_bracket;

use crate::brackets::Bracket;
use tax_bracket::TaxBrackets;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FilingStatus {
    Single,
    MarriedFilingSeparately,
    MarriedFilingJointly,
    QualifyingSurvivingSpouse,
    HeadOfHousehold,
}

//For get_tax_brackets fn
macro_rules! generate_get_tax_brackets {
    (
        $($state:literal => {
            $(
                $year:literal => {
                    $($status:path => $func:path,)*
                },
            )*
        },)*
    ) => {
        fn get_tax_brackets (state: &str, year: i32, filing_status: FilingStatus) -> Vec<Bracket> {
            let state_lower = state.to_lowercase();
            match state_lower.as_str() {
                $(
                    $state => match year {
                        $(
                            $year => {
                                match filing_status {
                                    $($status => $func(),)*
                                }
                            }
                        )*
                        _ => {
                            eprintln! ("Year {} not supported for {}. Defaulting to the latest supported year's Single bracket.", year, $state);
                            // Directly call the function associated with Single for the latest year
                            $(
                                $(
                                    match $year { // Use a dummy match on year to trigger the inner loop once
                                        $year => {
                                            match FilingStatus::Single {
                                                $status => return $func(),
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                )*
                            )*
                            unreachable!("Latest year's Single filing status should be defined");
                        }
                    },
                )*
                _=>{
                    eprintln! ("Error: State '{}' is not currently supported.", state);
                    vec![Bracket(f64::INFINITY, 0.0)]
                }
            }
        }
    };
}

generate_get_tax_brackets!(
    "ca" => {
        2024 => {
            FilingStatus::Single => brackets::ca_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ca_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ca_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ca_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ca_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ca_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ca_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ca_joint_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ca_joint_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ca_headhouse_tax_2025,
        },
    },
    "tx" => {
        2024 => {
            FilingStatus::Single => brackets::tx_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::tx_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::tx_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tx_single_tax,
            FilingStatus::HeadOfHousehold => brackets::tx_single_tax,
        },
        2025 => {
            FilingStatus::Single => brackets::tx_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::tx_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::tx_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tx_single_tax,
            FilingStatus::HeadOfHousehold => brackets::tx_single_tax,
        },
    },
    "tn" => {
        2024 => {
            FilingStatus::Single => brackets::tn_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::tn_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::tn_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tn_single_tax,
            FilingStatus::HeadOfHousehold => brackets::tn_single_tax,
        },
        2025 => {
            FilingStatus::Single => brackets::tn_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::tn_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::tn_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tn_single_tax,
            FilingStatus::HeadOfHousehold => brackets::tn_single_tax,
        },
    },
);

//calculates Income Tax based on income and filing status
pub fn calculate_income_tax(
    state: &str,
    income: f64,
    filing_status: FilingStatus,
    year: i32,
) -> f64 {
    let brackets = get_tax_brackets(state, year, filing_status);
    TaxBrackets::new(brackets).taxes(income)
}

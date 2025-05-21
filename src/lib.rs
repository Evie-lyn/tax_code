#[macro_use]
mod brackets;
mod tax_bracket;
mod deductions;
mod income_based_deduction;

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
                        
                            $(
                                $(
                                    match $year { 
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

    "al" => { //Alabama
        2024 => {
            FilingStatus::Single => brackets::al_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::al_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::al_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::al_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::al_headhouse_tax_2024,
        },
    },
    "ak" => { //Alaska
        2024 => {
            FilingStatus::Single => brackets::ak_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::ak_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::ak_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ak_single_tax,
            FilingStatus::HeadOfHousehold => brackets::ak_single_tax,
        },
        2025 => {
            FilingStatus::Single => brackets::ak_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::ak_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::ak_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ak_single_tax,
            FilingStatus::HeadOfHousehold => brackets::ak_single_tax,
        },
    },

    "az" => { //Arizona
        2024 => {
            FilingStatus::Single => brackets::az_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::az_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::az_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::az_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::az_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::az_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::az_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::az_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::az_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::az_single_tax_2025,
        },
    },

    "ar" => { //Arkansas
        2024 => {
            FilingStatus::Single => brackets::ar_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ar_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ar_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ar_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ar_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ar_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ar_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ar_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ar_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ar_single_tax_2025,
        },
    },

    "ca" => { //California
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

    "co" => { //Colorado
        2024 => {
            FilingStatus::Single => brackets::co_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::co_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::co_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::co_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::co_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::co_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::co_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::co_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::co_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::co_single_tax_2025,
        },
    },

    "ct" => { //Connecticut
        2024 => {
            FilingStatus::Single => brackets::ct_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ct_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ct_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ct_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ct_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ct_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ct_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ct_joint_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ct_joint_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ct_headhouse_tax_2025,
        },
    },

    "de" => { //Delaware
        2024 => {
            FilingStatus::Single => brackets::de_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::de_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::de_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::de_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::de_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::de_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::de_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::de_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::de_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::de_single_tax_2025,
        },
    },

    "fl" => { //Florida
        2024 => {
            FilingStatus::Single => brackets::fl_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::fl_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::fl_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::fl_single_tax,
            FilingStatus::HeadOfHousehold => brackets::fl_single_tax,
        },
        2025 => {
            FilingStatus::Single => brackets::fl_single_tax,
            FilingStatus::MarriedFilingSeparately => brackets::fl_single_tax,
            FilingStatus::MarriedFilingJointly => brackets::fl_single_tax,
            FilingStatus::QualifyingSurvivingSpouse => brackets::fl_single_tax,
            FilingStatus::HeadOfHousehold => brackets::fl_single_tax,
        },
    },

    "ga" => { //Georgia
        2024 => {
            FilingStatus::Single => brackets::ga_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ga_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ga_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ga_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ga_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ga_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ga_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ga_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ga_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ga_single_tax_2025,
        },
    },

    "hi" => { //Hawaii
        2024 => {
            FilingStatus::Single => brackets::hi_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::hi_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::hi_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::hi_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::hi_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::hi_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::hi_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::hi_joint_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::hi_joint_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::hi_headhouse_tax_2025,
        },
    },

    "id" => { //Idaho
        2024 => {
            FilingStatus::Single => brackets::id_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::id_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::id_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::id_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::id_headhouse_tax_2024,
        },
    },

    "il" => { //Illinois
        2024 => {
            FilingStatus::Single => brackets::il_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::il_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::il_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::il_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::il_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::il_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::il_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::il_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::il_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::il_single_tax_2025,
        },
    },

    "in" => { //Indiana
        2024 => {
            FilingStatus::Single => brackets::in_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::in_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::in_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::in_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::in_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::in_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::in_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::in_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::in_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::in_single_tax_2025,
        },
    },

    "ia" => { //Iowa
        2024 => {
            FilingStatus::Single => brackets::ia_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ia_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ia_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ia_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ia_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ia_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ia_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ia_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ia_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ia_single_tax_2025,
        },
    },

    "ks" => { //Kansas
        2024 => {
            FilingStatus::Single => brackets::ks_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ks_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ks_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ks_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ks_headhouse_tax_2024,
        },
    },

    "ky" => { //Kentucky
        2024 => {
            FilingStatus::Single => brackets::ky_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ky_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ky_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ky_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ky_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ky_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ky_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ky_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ky_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ky_single_tax_2025,
        },
    },

    "la" => { //Louisiana
        2024 => {
            FilingStatus::Single => brackets::la_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::la_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::la_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::la_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::la_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::la_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::la_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::la_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::la_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::la_single_tax_2025,
        },
    },

    "me" => { //Maine
        2024 => {
            FilingStatus::Single => brackets::me_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::me_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::me_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::me_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::me_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::me_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::me_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::me_joint_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::me_joint_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::me_headhouse_tax_2025,
        },
    },

    "md" => { //Maryland
        2024 => {
            FilingStatus::Single => brackets::md_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::md_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::md_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::md_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::md_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::md_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::md_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::md_joint_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::md_joint_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::md_headhouse_tax_2025,
        },
    },

    "ma" => { //Massachusetts
        2024 => {
            FilingStatus::Single => brackets::ma_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ma_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ma_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ma_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ma_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ma_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ma_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ma_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ma_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ma_single_tax_2025,
        },
    },

    "mi" => { //Michigan
        2024 => {
            FilingStatus::Single => brackets::mi_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::mi_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::mi_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mi_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::mi_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::mi_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::mi_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::mi_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mi_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::mi_single_tax_2025,
        },
    },

    "mn" => { //Minnesota
        2024 => {
            FilingStatus::Single => brackets::mn_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::mn_separate_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::mn_joint_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mn_joint_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::mn_headhouse_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::mn_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::mn_separate_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::mn_joint_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mn_joint_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::mn_headhouse_tax_2025,
        },
    },

    "ms" => { //Mississippi
        2024 => {
            FilingStatus::Single => brackets::ms_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ms_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ms_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ms_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ms_single_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ms_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ms_single_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ms_single_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ms_single_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ms_single_tax_2025,
        },
    },

    "mo" => { //Missouri (tax same across filing status)
        2024 => {
            FilingStatus::Single => brackets::mo_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::mo_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::mo_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mo_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::mo_single_tax_2024,
        },
    },

    "tx" => { //Texas
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

    "tn" => { //Tennesse
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

/// For deductions
macro_rules! generate_get_deductions {
    (
        $($state:literal => {
            $(
                $year:literal => {
                    $($status:path => $func:path,)*
                },
            )*
        },)*
    ) => {
        pub fn get_deductions (state: &str, year: i32, filing_status: FilingStatus, income: f64) -> deductions::Deduction {
            let state_lower = state.to_lowercase();
            match state_lower.as_str() {
                $(
                    $state => match year {
                        $(
                            $year => {
                                match filing_status {
                                    $($status => $func(income, &filing_status),)*
                                }
                            }
                        )*
                        _ => {
                            eprintln! ("Year {} not supported for {}. Defaulting to latest supported year's Single deduction.", year, $state);

                            $(
                                $(
                                    match $year {
                                        $year => {
                                            return $func(income, &FilingStatus::Single);
                                        }
                                        _ => {}
                                    }
                                )*
                            )*
                            unreachable!("Latest year's Single filing status deduction should be defined");
                        }
                    },
                )*
                _=>{
                    eprintln! ("Error: State '{}' is not currently supported for deductions. Defaulting to 0 deduction.", state);
                    deductions::Deduction { standard_deduction: 0.0 }
                }
            }
        }
    };
}

generate_get_deductions!(
    "al" => { // Alabama
        2024 => {
            FilingStatus::Single => income_based_deduction::al_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => income_based_deduction::al_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => income_based_deduction::al_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => income_based_deduction::al_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => income_based_deduction::al_standard_deduction_2024,
        },
    },
    "ak" => { // Alaska
        2024 => {
            FilingStatus::Single => deductions::ak_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::ak_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::ak_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ak_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::ak_standard_deduction,
        },
        2025 => {
            FilingStatus::Single => deductions::ak_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::ak_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::ak_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ak_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::ak_standard_deduction,
        },
    },

    "az" => { // Arizona
        2024 => {
            FilingStatus::Single => deductions::az_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::az_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::az_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::az_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::az_standard_deduction_2024,
        },
    },

    "ar" => { // Arkansas
        2024 => {
            FilingStatus::Single => deductions::ar_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ar_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ar_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ar_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ar_standard_deduction_2024,
        },
    },
    
    "ca" => { // California
        2024 => {
            FilingStatus::Single => deductions::ca_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ca_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ca_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ca_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ca_standard_deduction_2024,
        },
    },

    "co" => { // Colorado
        2024 => {
            FilingStatus::Single => deductions::co_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::co_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::co_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::co_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::co_standard_deduction,
        },
        2025 => {
            FilingStatus::Single => deductions::co_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::co_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::co_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::co_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::co_standard_deduction,
        },
    },

    "ct" => { // Connecticut
        2024 => {
            FilingStatus::Single => deductions::ct_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::ct_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::ct_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ct_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::ct_standard_deduction,
        },
        2025 => {
            FilingStatus::Single => deductions::ct_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::ct_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::ct_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ct_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::ct_standard_deduction,
        },
    },

    "de" => { // Delaware
        2024 => {
            FilingStatus::Single => deductions::de_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::de_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::de_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::de_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::de_standard_deduction_2024,
        },
    },

    "fl" => { // Florida
        2024 => {
            FilingStatus::Single => deductions::fl_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::fl_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::fl_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::fl_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::fl_standard_deduction,
        },
        2025 => {
            FilingStatus::Single => deductions::fl_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::fl_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::fl_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::fl_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::fl_standard_deduction,
        },
    },

    "ga" => { // Georggia
        2024 => {
            FilingStatus::Single => deductions::ga_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ga_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ga_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ga_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ga_standard_deduction_2024,
        },
    },

    "hi" => { // Hawaii
        2024 => {
            FilingStatus::Single => deductions::hi_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::hi_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::hi_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::hi_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::hi_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::hi_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::hi_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::hi_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::hi_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::hi_standard_deduction_2025,
        },
    },

    "id" => { // Idaho
        2024 => {
            FilingStatus::Single => deductions::id_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::id_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::id_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::id_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::id_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::id_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::id_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::id_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::id_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::id_standard_deduction_2025,
        },
    },

    "il" => { // Illinois
        2024 => {
            FilingStatus::Single => deductions::il_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::il_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::il_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::il_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::il_standard_deduction,
        },
        2025 => {
            FilingStatus::Single => deductions::il_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::il_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::il_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::il_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::il_standard_deduction,
        },
    },

    "in" => { // Indiana
        2024 => {
            FilingStatus::Single => deductions::in_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::in_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::in_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::in_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::in_standard_deduction,
        },
        2025 => {
            FilingStatus::Single => deductions::in_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::in_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::in_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::in_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::in_standard_deduction,
        },
    },

    "ia" => { // Iowa
        2024 => {
            FilingStatus::Single => deductions::ia_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ia_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ia_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ia_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ia_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ia_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ia_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ia_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ia_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ia_standard_deduction_2025,
        },
    },

    "ks" => { // Kansas
        2024 => {
            FilingStatus::Single => deductions::ks_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ks_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ks_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ks_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ks_standard_deduction_2024,
        },
    },

    "ky" => { // Kentucky
        2024 => {
            FilingStatus::Single => deductions::ky_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ky_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ky_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ky_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ky_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ky_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ky_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ky_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ky_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ky_standard_deduction_2025,
        },
    },

    "la" => { // Louisiana
        2024 => {
            FilingStatus::Single => deductions::la_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::la_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::la_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::la_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::la_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::la_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::la_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::la_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::la_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::la_standard_deduction_2025,
        },
    },

    "me" => { // Maine
        2024 => {
            FilingStatus::Single => deductions::me_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::me_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::me_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::me_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::me_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::me_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::me_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::me_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::me_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::me_standard_deduction_2025,
        },
    },

    "md" => { // Maryland
        2024 => {
            FilingStatus::Single => deductions::md_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::md_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::md_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::md_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::md_standard_deduction_2024,
        },
    },

    "md" => { // Massachusetts
        2024 => {
            FilingStatus::Single => deductions::ma_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::ma_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::ma_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ma_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::ma_standard_deduction,
        },
    },

    "mi" => { // Michigan
        2024 => {
            FilingStatus::Single => deductions::mi_standard_deduction,
            FilingStatus::MarriedFilingSeparately => deductions::mi_standard_deduction,
            FilingStatus::MarriedFilingJointly => deductions::mi_standard_deduction,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mi_standard_deduction,
            FilingStatus::HeadOfHousehold => deductions::mi_standard_deduction,
        },
    },

    "mn" => { // Minnesota
        2024 => {
            FilingStatus::Single => deductions::mn_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::mn_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::mn_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mn_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::mn_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::mn_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::mn_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::mn_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mn_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::mn_standard_deduction_2025,
        },
    },

    "ms" => { // Mississippi
        2024 => {
            FilingStatus::Single => deductions::ms_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ms_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ms_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ms_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ms_standard_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ms_standard_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ms_standard_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ms_standard_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ms_standard_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ms_standard_deduction_2025,
        },
    },

    "mo" => { // Missouri
        2024 => {
            FilingStatus::Single => deductions::mo_standard_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::mo_standard_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::mo_standard_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mo_standard_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::mo_standard_deduction_2024,
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
    let deduction_amount = crate::get_deductions (state, year, filing_status, income).standard_deduction;
    let taxable_income = (income - deduction_amount).max (0.0);

    let brackets = crate::get_tax_brackets(state, year, filing_status);
    TaxBrackets::new(brackets).taxes(taxable_income)
}

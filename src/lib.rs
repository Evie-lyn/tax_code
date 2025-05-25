pub mod brackets;

pub mod tax_bracket; 
pub mod deductions;  
pub mod income_based_deduction; 

use brackets::Bracket;
use crate::tax_bracket::TaxBrackets;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Deduction {
    pub standard_deduction: f64
}
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
        fn get_tax_brackets (state: &str, year: i32, filing_status: &FilingStatus) -> Vec<Bracket> {
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
        FilingStatus::MarriedFilingSeparately => brackets::al_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::al_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::al_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::al_head_of_household_tax_2024, 
    },
},
"ak" => { //Alaska
    2024 => {
        FilingStatus::Single => brackets::ak_single_tax_2024, 
        FilingStatus::MarriedFilingSeparately => brackets::ak_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::ak_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::ak_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::ak_head_of_household_tax_2024, 
    },
    2025 => {
        FilingStatus::Single => brackets::ak_single_tax_2025, 
        FilingStatus::MarriedFilingSeparately => brackets::ak_married_filing_separately_tax_2025, 
        FilingStatus::MarriedFilingJointly => brackets::ak_married_filing_jointly_tax_2025, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::ak_qualifying_surviving_spouse_tax_2025, 
        FilingStatus::HeadOfHousehold => brackets::ak_head_of_household_tax_2025, 
    },
},

"az" => { //Arizona
    2024 => {
        FilingStatus::Single => brackets::az_single_tax_2024, 
        FilingStatus::MarriedFilingSeparately => brackets::az_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::az_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::az_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::az_head_of_household_tax_2024, 
    },
    2025 => {
        FilingStatus::Single => brackets::az_single_tax_2025, 
        FilingStatus::MarriedFilingSeparately => brackets::az_married_filing_separately_tax_2025, 
        FilingStatus::MarriedFilingJointly => brackets::az_married_filing_jointly_tax_2025, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::az_qualifying_surviving_spouse_tax_2025, 
        FilingStatus::HeadOfHousehold => brackets::az_head_of_household_tax_2025, 
    },
},

"ar" => { //Arkansas
    2024 => {
        FilingStatus::Single => brackets::ar_single_tax_2024, 
        FilingStatus::MarriedFilingSeparately => brackets::ar_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::ar_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::ar_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::ar_head_of_household_tax_2024, 
    },
    2025 => {
        FilingStatus::Single => brackets::ar_single_tax_2025, 
        FilingStatus::MarriedFilingSeparately => brackets::ar_married_filing_separately_tax_2025, 
        FilingStatus::MarriedFilingJointly => brackets::ar_married_filing_jointly_tax_2025, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::ar_qualifying_surviving_spouse_tax_2025, 
        FilingStatus::HeadOfHousehold => brackets::ar_head_of_household_tax_2025, 
    },
},

"ca" => { //California
    2024 => {
        FilingStatus::Single => brackets::ca_single_tax_2024, 
        FilingStatus::MarriedFilingSeparately => brackets::ca_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::ca_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::ca_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::ca_head_of_household_tax_2024, 
    },
},

"co" => { //Colorado
    2024 => {
        FilingStatus::Single => brackets::co_single_tax_2024, 
        FilingStatus::MarriedFilingSeparately => brackets::co_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::co_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::co_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::co_head_of_household_tax_2024, 
    },
    2025 => {
        FilingStatus::Single => brackets::co_single_tax_2025, 
        FilingStatus::MarriedFilingSeparately => brackets::co_married_filing_separately_tax_2025, 
        FilingStatus::MarriedFilingJointly => brackets::co_married_filing_jointly_tax_2025, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::co_qualifying_surviving_spouse_tax_2025, 
        FilingStatus::HeadOfHousehold => brackets::co_head_of_household_tax_2025, 
    },
},

"ct" => { //Connecticut
    2024 => {
        FilingStatus::Single => brackets::ct_single_tax_2024, 
        FilingStatus::MarriedFilingSeparately => brackets::ct_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::ct_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::ct_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::ct_head_of_household_tax_2024, 
    },
    2025 => {
        FilingStatus::Single => brackets::ct_single_tax_2025, 
        FilingStatus::MarriedFilingSeparately => brackets::ct_married_filing_separately_tax_2025, 
        FilingStatus::MarriedFilingJointly => brackets::ct_married_filing_jointly_tax_2025, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::ct_qualifying_surviving_spouse_tax_2025, 
        FilingStatus::HeadOfHousehold => brackets::ct_head_of_household_tax_2025, 
    },
},

"de" => { //Delaware
    2024 => {
        FilingStatus::Single => brackets::de_single_tax_2024, 
        FilingStatus::MarriedFilingSeparately => brackets::de_married_filing_separately_tax_2024, 
        FilingStatus::MarriedFilingJointly => brackets::de_married_filing_jointly_tax_2024, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::de_qualifying_surviving_spouse_tax_2024, 
        FilingStatus::HeadOfHousehold => brackets::de_head_of_household_tax_2024, 
    },
    2025 => {
        FilingStatus::Single => brackets::de_single_tax_2025, 
        FilingStatus::MarriedFilingSeparately => brackets::de_married_filing_separately_tax_2025, 
        FilingStatus::MarriedFilingJointly => brackets::de_married_filing_jointly_tax_2025, 
        FilingStatus::QualifyingSurvivingSpouse => brackets::de_qualifying_surviving_spouse_tax_2025, 
        FilingStatus::HeadOfHousehold => brackets::de_head_of_household_tax_2025, 
    },
},

"fl" => {
    2024 => {
        FilingStatus::Single => brackets::fl_single_tax_2024,
        FilingStatus::MarriedFilingSeparately => brackets::fl_married_filing_separately_tax_2024,
        FilingStatus::MarriedFilingJointly => brackets::fl_married_filing_jointly_tax_2024,
        FilingStatus::QualifyingSurvivingSpouse => brackets::fl_qualifying_surviving_spouse_tax_2024,
        FilingStatus::HeadOfHousehold => brackets::fl_head_of_household_tax_2024,
    },
    2025 => {
        FilingStatus::Single => brackets::fl_single_tax_2025,
        FilingStatus::MarriedFilingSeparately => brackets::fl_married_filing_separately_tax_2025,
        FilingStatus::MarriedFilingJointly => brackets::fl_married_filing_jointly_tax_2025,
        FilingStatus::QualifyingSurvivingSpouse => brackets::fl_qualifying_surviving_spouse_tax_2025,
        FilingStatus::HeadOfHousehold => brackets::fl_head_of_household_tax_2025,
    },
},

"ga" => { //Georgia
    2024 => {
        FilingStatus::Single => brackets::ga_single_tax_2024,
        FilingStatus::MarriedFilingSeparately => brackets::ga_married_filing_separately_tax_2024,
        FilingStatus::MarriedFilingJointly => brackets::ga_married_filing_jointly_tax_2024,
        FilingStatus::QualifyingSurvivingSpouse => brackets::ga_qualifying_surviving_spouse_tax_2024,
        FilingStatus::HeadOfHousehold => brackets::ga_head_of_household_tax_2024,
    },
    2025 => {
        FilingStatus::Single => brackets::ga_single_tax_2025,
        FilingStatus::MarriedFilingSeparately => brackets::ga_married_filing_separately_tax_2025,
        FilingStatus::MarriedFilingJointly => brackets::ga_married_filing_jointly_tax_2025,
        FilingStatus::QualifyingSurvivingSpouse => brackets::ga_qualifying_surviving_spouse_tax_2025,
        FilingStatus::HeadOfHousehold => brackets::ga_head_of_household_tax_2025,
    },
},

"hi" => { //Hawaii
    2024 => {
        FilingStatus::Single => brackets::hi_single_tax_2024,
        FilingStatus::MarriedFilingSeparately => brackets::hi_married_filing_separately_tax_2024,
        FilingStatus::MarriedFilingJointly => brackets::hi_married_filing_jointly_tax_2024,
        FilingStatus::QualifyingSurvivingSpouse => brackets::hi_qualifying_surviving_spouse_tax_2024,
        FilingStatus::HeadOfHousehold => brackets::hi_head_of_household_tax_2024,
    },
    2025 => {
        FilingStatus::Single => brackets::hi_single_tax_2025,
        FilingStatus::MarriedFilingSeparately => brackets::hi_married_filing_separately_tax_2025,
        FilingStatus::MarriedFilingJointly => brackets::hi_married_filing_jointly_tax_2025,
        FilingStatus::QualifyingSurvivingSpouse => brackets::hi_qualifying_surviving_spouse_tax_2025,
        FilingStatus::HeadOfHousehold => brackets::hi_head_of_household_tax_2025,
    },
},

"id" => { //Idaho
    2024 => {
        FilingStatus::Single => brackets::id_single_tax_2024,
        FilingStatus::MarriedFilingSeparately => brackets::id_married_filing_separately_tax_2024,
        FilingStatus::MarriedFilingJointly => brackets::id_married_filing_jointly_tax_2024,
        FilingStatus::QualifyingSurvivingSpouse => brackets::id_qualifying_surviving_spouse_tax_2024,
        FilingStatus::HeadOfHousehold => brackets::id_head_of_household_tax_2024,
    },
},

"il" => { //Illinois
    2024 => {
        FilingStatus::Single => brackets::il_single_tax_2024,
        FilingStatus::MarriedFilingSeparately => brackets::il_married_filing_separately_tax_2024,
        FilingStatus::MarriedFilingJointly => brackets::il_married_filing_jointly_tax_2024,
        FilingStatus::QualifyingSurvivingSpouse => brackets::il_qualifying_surviving_spouse_tax_2024,
        FilingStatus::HeadOfHousehold => brackets::il_head_of_household_tax_2024,
    },
    2025 => {
        FilingStatus::Single => brackets::il_single_tax_2025,
        FilingStatus::MarriedFilingSeparately => brackets::il_married_filing_separately_tax_2025,
        FilingStatus::MarriedFilingJointly => brackets::il_married_filing_jointly_tax_2025,
        FilingStatus::QualifyingSurvivingSpouse => brackets::il_qualifying_surviving_spouse_tax_2025,
        FilingStatus::HeadOfHousehold => brackets::il_head_of_household_tax_2025,
    },
},

"in" => { //Indiana
    2024 => {
        FilingStatus::Single => brackets::in_single_tax_2024,
        FilingStatus::MarriedFilingSeparately => brackets::in_married_filing_separately_tax_2024,
        FilingStatus::MarriedFilingJointly => brackets::in_married_filing_jointly_tax_2024,
        FilingStatus::QualifyingSurvivingSpouse => brackets::in_qualifying_surviving_spouse_tax_2024,
        FilingStatus::HeadOfHousehold => brackets::in_head_of_household_tax_2024,
    },
    2025 => {
        FilingStatus::Single => brackets::in_single_tax_2025,
        FilingStatus::MarriedFilingSeparately => brackets::in_married_filing_separately_tax_2025,
        FilingStatus::MarriedFilingJointly => brackets::in_married_filing_jointly_tax_2025,
        FilingStatus::QualifyingSurvivingSpouse => brackets::in_qualifying_surviving_spouse_tax_2025,
        FilingStatus::HeadOfHousehold => brackets::in_head_of_household_tax_2025,
    },
},

"ia" => { //Iowa
        2024 => {
            FilingStatus::Single => brackets::ia_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ia_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ia_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ia_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ia_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ia_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ia_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ia_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ia_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ia_head_of_household_tax_2025,
        },
    },

    "ks" => { //Kansas
        2024 => {
            FilingStatus::Single => brackets::ks_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ks_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ks_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ks_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ks_head_of_household_tax_2024,
        },
    },

    "ky" => { //Kentucky
        2024 => {
            FilingStatus::Single => brackets::ky_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ky_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ky_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ky_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ky_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ky_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ky_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ky_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ky_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ky_head_of_household_tax_2025,
        },
    },

    "la" => { //Louisiana
        2024 => {
            FilingStatus::Single => brackets::la_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::la_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::la_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::la_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::la_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::la_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::la_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::la_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::la_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::la_head_of_household_tax_2025,
        },
    },

    "me" => { //Maine
        2024 => {
            FilingStatus::Single => brackets::me_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::me_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::me_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::me_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::me_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::me_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::me_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::me_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::me_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::me_head_of_household_tax_2025,
        },
    },

    "md" => { //Maryland
        2024 => {
            FilingStatus::Single => brackets::md_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::md_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::md_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::md_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::md_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::md_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::md_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::md_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::md_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::md_head_of_household_tax_2025,
        },
    },

    "ma" => { //Massachusetts
        2024 => {
            FilingStatus::Single => brackets::ma_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ma_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ma_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ma_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ma_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ma_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ma_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ma_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ma_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ma_head_of_household_tax_2025,
        },
    },

    "mi" => { //Michigan
        2024 => {
            FilingStatus::Single => brackets::mi_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::mi_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::mi_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mi_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::mi_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::mi_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::mi_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::mi_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mi_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::mi_head_of_household_tax_2025,
        },
    },

 "mn" => { //Minnesota
        2024 => {
            FilingStatus::Single => brackets::mn_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::mn_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::mn_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mn_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::mn_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::mn_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::mn_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::mn_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mn_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::mn_head_of_household_tax_2025,
        },
    },

    "ms" => { //Mississippi
        2024 => {
            FilingStatus::Single => brackets::ms_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ms_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ms_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ms_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ms_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ms_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ms_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ms_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ms_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ms_head_of_household_tax_2025,
        },
    },

    "mo" => {
        2024 => {
            FilingStatus::Single => brackets::mo_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::mo_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::mo_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mo_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::mo_single_tax_2024,
        },
    },

    "mt" => {
        2024 => {
            FilingStatus::Single => brackets::mt_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::mt_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::mt_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::mt_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::mt_single_tax_2024,
        },
    },

    "ne" => { //Nebraska
        2024 => {
            FilingStatus::Single => brackets::ne_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ne_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ne_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ne_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ne_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::ne_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::ne_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::ne_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ne_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::ne_head_of_household_tax_2025,
        },
    },

    "nv" => { //Nebraska
        2024 => {
            FilingStatus::Single => brackets::nv_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::nv_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::nv_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nv_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::nv_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::nv_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::nv_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::nv_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nv_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::nv_head_of_household_tax_2025,
        },
    },

    "nh" => { //New Hampshire
        2024 => {
            FilingStatus::Single => brackets::nh_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::nh_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::nh_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nh_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::nh_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::nh_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::nh_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::nh_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nh_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::nh_head_of_household_tax_2025,
        },
    },

    "nj" => { //New Jersey
        2024 => {
            FilingStatus::Single => brackets::nj_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::nj_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::nj_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nj_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::nj_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::nj_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::nj_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::nj_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nj_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::nj_head_of_household_tax_2025,
        },
    },

    "nm" => { //New Mexico
        2024 => {
            FilingStatus::Single => brackets::nm_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::nm_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::nm_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nm_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::nm_single_tax_2024,
        },
    },

    "ny" => { //New York
        2024 => {
            FilingStatus::Single => brackets::ny_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::ny_single_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::ny_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::ny_single_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::ny_single_tax_2024,
        },
    },

    "nc" => { //North Carolina
        2024 => {
            FilingStatus::Single => brackets::nc_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::nc_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::nc_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nc_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::nc_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::nc_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::nc_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::nc_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nc_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::nc_head_of_household_tax_2025,
        },
    },

    "nd" => { //North Dakota
        2024 => {
            FilingStatus::Single => brackets::nd_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::nd_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::nd_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nd_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::nd_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::nd_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::nd_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::nd_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::nd_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::nd_head_of_household_tax_2025,
        },
    },

    "oh" => { // Ohio
        2024 => {
            FilingStatus::Single => brackets::oh_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::oh_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::oh_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::oh_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::oh_head_of_household_tax_2024,
        },
    },

    "tx" => {
        2024 => {
            FilingStatus::Single => brackets::tx_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::tx_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::tx_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tx_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::tx_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::tx_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::tx_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::tx_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tx_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::tx_head_of_household_tax_2025,
        },
    },

    "tn" => {
        2024 => {
            FilingStatus::Single => brackets::tn_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => brackets::tn_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => brackets::tn_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tn_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => brackets::tn_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => brackets::tn_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => brackets::tn_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => brackets::tn_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => brackets::tn_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => brackets::tn_head_of_household_tax_2025,
        },
    },
);

// For deductions
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
        pub fn get_deductions (state: &str, year: i32, filing_status: &FilingStatus, income: f64) -> Deduction {
            let state_lower = state.to_lowercase();
            match state_lower.as_str() {
                $(
                    $state => match year {
                        $(
                            $year => {
                                match *filing_status { 
                                    $($status => $func(income, filing_status)),* 
                                }
                            }
                        )*
                        _ => {
                            eprintln! ("Year {} not supported for {}. Defaulting to standard deduction for Single filing status.", year, $state);
                            Deduction { standard_deduction: 0.0 }
                        }
                    },
                )*
                _=>{
                    eprintln! ("Error: State '{}' is not currently supported for deductions. Defaulting to 0 deduction.", state);
                    Deduction { standard_deduction: 0.0 }
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
            FilingStatus::Single => deductions::ak_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ak_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ak_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ak_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ak_head_of_household_deduction_2024,
        },
    },

    "az" => { // Arizona
        2024 => {
            FilingStatus::Single => deductions::az_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::az_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::az_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::az_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::az_head_of_household_deduction_2024,
        },
    },

    "ar" => { // Arkansas
        2024 => {
            FilingStatus::Single => deductions::ar_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ar_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ar_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ar_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ar_head_of_household_deduction_2024,
        },
    },

    "ca" => { // California
        2024 => {
            FilingStatus::Single => deductions::ca_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ca_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ca_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ca_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ca_head_of_household_deduction_2024,
        },
    },

    "co" => { // Colorado
        2024 => {
            FilingStatus::Single => deductions::co_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::co_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::co_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::co_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::co_head_of_household_deduction_2024,
        },
    },

    "ct" => { // Connecticut
        2024 => {
            FilingStatus::Single => deductions::ct_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ct_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ct_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ct_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ct_head_of_household_deduction_2024,
        },
    },

    "de" => { // Delaware
        2024 => {
            FilingStatus::Single => deductions::de_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::de_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::de_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::de_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::de_head_of_household_deduction_2024,
        },
    },

    "fl" => { // Florida
        2024 => {
            FilingStatus::Single => deductions::fl_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::fl_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::fl_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::fl_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::fl_head_of_household_deduction_2024,
        },
    },

    "ga" => { // Georgia
        2024 => {
            FilingStatus::Single => deductions::ga_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ga_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ga_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ga_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ga_head_of_household_deduction_2024,
        },
    },

    "hi" => { // Hawaii
        2024 => {
            FilingStatus::Single => deductions::hi_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::hi_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::hi_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::hi_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::hi_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::hi_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::hi_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::hi_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::hi_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::hi_head_of_household_deduction_2025,
        },
    },

    "id" => { // Idaho
        2024 => {
            FilingStatus::Single => deductions::id_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::id_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::id_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::id_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::id_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::id_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::id_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::id_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::id_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::id_head_of_household_deduction_2025,
        },
    },

  "il" => { // Illinois
        2024 => {
            FilingStatus::Single => deductions::il_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::il_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::il_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::il_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::il_head_of_household_deduction_2024,
        },
    },

    "in" => { // Indiana
        2024 => {
            FilingStatus::Single => deductions::in_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::in_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::in_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::in_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::in_head_of_household_deduction_2024,
        },
    },

    "ia" => { // Iowa
        2024 => {
            FilingStatus::Single => deductions::ia_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ia_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ia_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ia_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ia_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ia_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ia_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ia_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ia_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ia_head_of_household_deduction_2025,
        },
    },

    "ks" => { // Kansas
        2024 => {
            FilingStatus::Single => deductions::ks_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ks_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ks_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ks_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ks_head_of_household_deduction_2024,
        },
    },

    "ky" => { // Kentucky
        2024 => {
            FilingStatus::Single => deductions::ky_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ky_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ky_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ky_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ky_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ky_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ky_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ky_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ky_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ky_head_of_household_deduction_2025,
        },
    },

    "la" => { // Louisiana
        2024 => {
            FilingStatus::Single => deductions::la_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::la_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::la_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::la_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::la_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::la_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::la_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::la_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::la_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::la_head_of_household_deduction_2025,
        },
    },

    "me" => { // Maine
        2024 => {
            FilingStatus::Single => deductions::me_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::me_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::me_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::me_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::me_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::me_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::me_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::me_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::me_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::me_head_of_household_deduction_2025,
        },
    },

    "md" => { // Maryland
        2024 => {
            FilingStatus::Single => deductions::md_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::md_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::md_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::md_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::md_head_of_household_deduction_2024,
        },
    },

    "ma" => { // Massachusetts
        2024 => {
            FilingStatus::Single => deductions::ma_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ma_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ma_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ma_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ma_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ma_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ma_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ma_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ma_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ma_head_of_household_deduction_2025,
        },
    },

    "mi" => { // Michigan
        2024 => {
            FilingStatus::Single => deductions::mi_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::mi_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::mi_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mi_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::mi_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::mi_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::mi_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::mi_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mi_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::mi_head_of_household_deduction_2025,
        },
    },

   "mn" => { // Minnesota
        2024 => {
            FilingStatus::Single => deductions::mn_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::mn_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::mn_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mn_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::mn_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::mn_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::mn_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::mn_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mn_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::mn_head_of_household_deduction_2025,
        },
    },

    "ms" => { // Mississippi
        2024 => {
            FilingStatus::Single => deductions::ms_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ms_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ms_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ms_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ms_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ms_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ms_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ms_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ms_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ms_head_of_household_deduction_2025,
        },
    },

    "mo" => { // Missouri
        2024 => {
            FilingStatus::Single => deductions::mo_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::mo_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::mo_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mo_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::mo_head_of_household_deduction_2024,
        },
    },

    "mt" => { // Montana
        2024 => {
            FilingStatus::Single => deductions::mt_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::mt_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::mt_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mt_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::mt_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::mt_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::mt_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::mt_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::mt_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::mt_head_of_household_deduction_2025,
        },
    },

    "ne" => { // Nebraska
        2024 => {
            FilingStatus::Single => deductions::ne_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ne_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ne_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ne_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ne_head_of_household_deduction_2024,
        },
    },

    "nv" => { // Nevada
        2024 => {
            FilingStatus::Single => deductions::nv_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::nv_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::nv_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nv_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::nv_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::nv_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::nv_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::nv_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nv_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::nv_head_of_household_deduction_2025,
        },
    },

    "nh" => { // New Hampshire
        2024 => {
            FilingStatus::Single => deductions::nh_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::nh_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::nh_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nh_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::nh_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::nh_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::nh_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::nh_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nh_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::nh_head_of_household_deduction_2025,
        },
    },

    "nh" => { // New Jersey
        2024 => {
            FilingStatus::Single => deductions::nj_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::nj_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::nj_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nj_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::nj_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::nj_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::nj_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::nj_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nj_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::nj_head_of_household_deduction_2025,
        },
    },

    "nm" => { // New Mexico
        2024 => {
            FilingStatus::Single => deductions::nm_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::nm_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::nm_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nm_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::nm_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::nm_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::nm_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::nm_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nm_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::nm_head_of_household_deduction_2025,
        },
    },

    "ny" => { // New York
        2024 => {
            FilingStatus::Single => deductions::ny_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ny_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ny_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ny_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ny_head_of_household_deduction_2024,
        },
    },

    "nm" => { // North Carolina
        2024 => {
            FilingStatus::Single => deductions::nc_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::nc_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::nc_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nc_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::nc_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::nc_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::nc_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::nc_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nc_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::nc_head_of_household_deduction_2025,
        },
    },

    "nd" => { // North Dakota
        2024 => {
            FilingStatus::Single => deductions::nd_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::nd_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::nd_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nd_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::nd_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::nd_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::nd_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::nd_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::nd_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::nd_head_of_household_deduction_2025,
        },
    },

    "oh" => { // Ohio
        2024 => {
            FilingStatus::Single => deductions::oh_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::oh_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::oh_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::oh_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::oh_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::oh_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::oh_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::oh_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::oh_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::oh_head_of_household_deduction_2025,
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
    let deduction_amount = crate::get_deductions (state, year, &filing_status, income).standard_deduction;
    let taxable_income = (income - deduction_amount).max (0.0);

    let brackets = crate::get_tax_brackets(state, year, &filing_status);
    TaxBrackets::new(brackets).taxes(taxable_income)
}
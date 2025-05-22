pub mod generated_tax_data;

pub mod tax_bracket; 
pub mod deductions;  
pub mod income_based_deduction; 

use tax_bracket::TaxBrackets;

use generated_tax_data::Bracket;
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
            FilingStatus::Single => generated_tax_data::al_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::al_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::al_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::al_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::al_head_of_household_tax_2024,
        },
    },
    "ak" => { //Alaska
        2024 => {
            FilingStatus::Single => generated_tax_data::ak_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ak_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ak_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ak_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ak_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ak_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ak_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ak_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ak_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::ak_head_of_household_tax_2025,
        },
    },

    "az" => { //Arizona
        2024 => {
            FilingStatus::Single => generated_tax_data::az_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::az_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::az_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::az_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::az_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::az_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::az_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::az_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::az_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::az_head_of_household_tax_2025,
        },
    },

    "ar" => { //Arkansas
        2024 => {
            FilingStatus::Single => generated_tax_data::ar_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ar_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ar_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ar_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ar_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ar_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ar_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ar_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ar_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::ar_head_of_household_tax_2025,
        },
    },

    "ca" => { //California
        2024 => {
            FilingStatus::Single => generated_tax_data::ca_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ca_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ca_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ca_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ca_head_of_household_tax_2024,
        },
    },

    "co" => { //Colorado
        2024 => {
            FilingStatus::Single => generated_tax_data::co_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::co_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::co_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::co_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::co_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::co_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::co_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::co_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::co_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::co_head_of_household_tax_2025,
        },
    },

    "ct" => { //Connecticut
        2024 => {
            FilingStatus::Single => generated_tax_data::ct_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ct_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ct_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ct_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ct_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ct_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ct_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ct_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ct_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::ct_head_of_household_tax_2025,
        },
    },

    "de" => { //Delaware
        2024 => {
            FilingStatus::Single => generated_tax_data::de_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::de_married_filing_separately_tax_2024, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::de_married_filing_jointly_tax_2024, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::de_qualifying_surviving_spouse_tax_2024, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::de_head_of_household_tax_2024, // Assuming status-specific functions exist
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::de_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::de_married_filing_separately_tax_2025, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::de_married_filing_jointly_tax_2025, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::de_qualifying_surviving_spouse_tax_2025, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::de_head_of_household_tax_2025, // Assuming status-specific functions exist
        },
    },

    "fl" => { 
        2024 => {
            FilingStatus::Single => generated_tax_data::fl_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::fl_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::fl_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::fl_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::fl_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::fl_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::fl_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::fl_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::fl_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::fl_head_of_household_tax_2025,
        },
    },

    "ga" => { //Georgia
        2024 => {
            FilingStatus::Single => generated_tax_data::ga_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ga_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ga_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ga_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ga_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ga_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ga_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ga_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ga_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::ga_head_of_household_tax_2025,
        },
    },

    "hi" => { //Hawaii
        2024 => {
            FilingStatus::Single => generated_tax_data::hi_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::hi_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::hi_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::hi_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::hi_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::hi_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::hi_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::hi_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::hi_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::hi_head_of_household_tax_2025,
        },
    },

"id" => { //Idaho
        2024 => {
            FilingStatus::Single => generated_tax_data::id_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::id_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::id_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::id_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::id_head_of_household_tax_2024,
        },
    },

    "il" => { //Illinois
        2024 => {
            FilingStatus::Single => generated_tax_data::il_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::il_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::il_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::il_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::il_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::il_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::il_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::il_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::il_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::il_head_of_household_tax_2025,
        },
    },

    "in" => { //Indiana
        2024 => {
            FilingStatus::Single => generated_tax_data::in_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::in_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::in_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::in_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::in_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::in_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::in_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::in_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::in_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::in_head_of_household_tax_2025,
        },
    },

    "ia" => { //Iowa
        2024 => {
            FilingStatus::Single => generated_tax_data::ia_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ia_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ia_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ia_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ia_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ia_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ia_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ia_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ia_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::ia_head_of_household_tax_2025,
        },
    },

    "ks" => { //Kansas
        2024 => {
            FilingStatus::Single => generated_tax_data::ks_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ks_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ks_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ks_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ks_head_of_household_tax_2024,
        },
    },

    "ky" => { //Kentucky
        2024 => {
            FilingStatus::Single => generated_tax_data::ky_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ky_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ky_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ky_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ky_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ky_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ky_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ky_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ky_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::ky_head_of_household_tax_2025,
        },
    },

    "la" => { //Louisiana
        2024 => {
            FilingStatus::Single => generated_tax_data::la_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::la_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::la_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::la_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::la_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::la_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::la_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::la_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::la_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::la_head_of_household_tax_2025,
        },
    },

    "me" => { //Maine
        2024 => {
            FilingStatus::Single => generated_tax_data::me_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::me_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::me_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::me_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::me_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::me_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::me_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::me_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::me_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::me_head_of_household_tax_2025,
        },
    },

    "md" => { //Maryland
        2024 => {
            FilingStatus::Single => generated_tax_data::md_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::md_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::md_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::md_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::md_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::md_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::md_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::md_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::md_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::md_head_of_household_tax_2025,
        },
    },

    "ma" => { //Massachusetts
        2024 => {
            FilingStatus::Single => generated_tax_data::ma_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ma_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ma_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ma_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::ma_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ma_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ma_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::ma_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ma_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::ma_head_of_household_tax_2025,
        },
    },

    "mi" => { //Michigan
        2024 => {
            FilingStatus::Single => generated_tax_data::mi_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::mi_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::mi_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::mi_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::mi_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::mi_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::mi_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::mi_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::mi_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::mi_head_of_household_tax_2025,
        },
    },

    "mn" => { //Minnesota
        2024 => {
            FilingStatus::Single => generated_tax_data::mn_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::mn_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::mn_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::mn_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::mn_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::mn_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::mn_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::mn_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::mn_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::mn_head_of_household_tax_2025,
        },
    },

    "ms" => { //Mississippi
        2024 => {
            FilingStatus::Single => generated_tax_data::ms_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ms_married_filing_separately_tax_2024, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::ms_married_filing_jointly_tax_2024, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ms_qualifying_surviving_spouse_tax_2024, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::ms_head_of_household_tax_2024, // Assuming status-specific functions exist
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ms_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ms_married_filing_separately_tax_2025, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::ms_married_filing_jointly_tax_2025, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ms_qualifying_surviving_spouse_tax_2025, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::ms_head_of_household_tax_2025, // Assuming status-specific functions exist
        },
    },

    "mo" => { 
        2024 => {
            FilingStatus::Single => generated_tax_data::mo_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::mo_single_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::mo_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::mo_single_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::mo_single_tax_2024,
        },
    },

    "mt" => { 
        2024 => {
            FilingStatus::Single => generated_tax_data::mt_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::mt_single_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::mt_single_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::mt_single_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::mt_single_tax_2024,
        },
    },

    "ne" => { //Nebraska
        2024 => {
            FilingStatus::Single => generated_tax_data::ne_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ne_married_filing_separately_tax_2024, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::ne_married_filing_jointly_tax_2024, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ne_qualifying_surviving_spouse_tax_2024, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::ne_head_of_household_tax_2024, // Assuming status-specific functions exist
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::ne_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::ne_married_filing_separately_tax_2025, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::ne_married_filing_jointly_tax_2025, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::ne_qualifying_surviving_spouse_tax_2025, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::ne_head_of_household_tax_2025, // Assuming status-specific functions exist
        },
    },

    "nv" => { //Nebraska
        2024 => {
            FilingStatus::Single => generated_tax_data::nv_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::nv_married_filing_separately_tax_2024, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::nv_married_filing_jointly_tax_2024, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::nv_qualifying_surviving_spouse_tax_2024, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::nv_head_of_household_tax_2024, // Assuming status-specific functions exist
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::nv_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::nv_married_filing_separately_tax_2025, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::nv_married_filing_jointly_tax_2025, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::nv_qualifying_surviving_spouse_tax_2025, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::nv_head_of_household_tax_2025, // Assuming status-specific functions exist
        },
    },

    "nh" => { //New Hampshire
        2024 => {
            FilingStatus::Single => generated_tax_data::nh_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::nh_married_filing_separately_tax_2024, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::nh_married_filing_jointly_tax_2024, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::nh_qualifying_surviving_spouse_tax_2024, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::nh_head_of_household_tax_2024, // Assuming status-specific functions exist
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::nh_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::nh_married_filing_separately_tax_2025, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::nh_married_filing_jointly_tax_2025, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::nh_qualifying_surviving_spouse_tax_2025, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::nh_head_of_household_tax_2025, // Assuming status-specific functions exist
        },
    },

    "nj" => { //New Jersey
        2024 => {
            FilingStatus::Single => generated_tax_data::nj_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::nj_married_filing_separately_tax_2024, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::nj_married_filing_jointly_tax_2024, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::nj_qualifying_surviving_spouse_tax_2024, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::nj_head_of_household_tax_2024, // Assuming status-specific functions exist
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::nj_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::nj_married_filing_separately_tax_2025, // Assuming status-specific functions exist
            FilingStatus::MarriedFilingJointly => generated_tax_data::nj_married_filing_jointly_tax_2025, // Assuming status-specific functions exist
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::nj_qualifying_surviving_spouse_tax_2025, // Assuming status-specific functions exist
            FilingStatus::HeadOfHousehold => generated_tax_data::nj_head_of_household_tax_2025, // Assuming status-specific functions exist
        },
    },

    "tx" => { 
       
        2024 => {
            FilingStatus::Single => generated_tax_data::tx_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::tx_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::tx_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::tx_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::tx_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::tx_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::tx_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::tx_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::tx_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::tx_head_of_household_tax_2025,
        },
    },

    "tn" => { 
        2024 => {
            FilingStatus::Single => generated_tax_data::tn_single_tax_2024,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::tn_married_filing_separately_tax_2024,
            FilingStatus::MarriedFilingJointly => generated_tax_data::tn_married_filing_jointly_tax_2024,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::tn_qualifying_surviving_spouse_tax_2024,
            FilingStatus::HeadOfHousehold => generated_tax_data::tn_head_of_household_tax_2024,
        },
        2025 => {
            FilingStatus::Single => generated_tax_data::tn_single_tax_2025,
            FilingStatus::MarriedFilingSeparately => generated_tax_data::tn_married_filing_separately_tax_2025,
            FilingStatus::MarriedFilingJointly => generated_tax_data::tn_married_filing_jointly_tax_2025,
            FilingStatus::QualifyingSurvivingSpouse => generated_tax_data::tn_qualifying_surviving_spouse_tax_2025,
            FilingStatus::HeadOfHousehold => generated_tax_data::tn_head_of_household_tax_2025,
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

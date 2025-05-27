pub mod tax_bracket; 
pub mod deductions;  
pub mod income_based_deduction; 
pub mod brackets;
pub mod get_tax_brackets_impl;

use crate::tax_bracket::TaxBrackets;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Deduction {
    pub standard_deduction: f64
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FilingStatus {
    Single,
    MarriedFilingSeparately,
    MarriedFilingJointly,
    QualifyingSurvivingSpouse,
    HeadOfHousehold,
}
pub fn calculate_income_tax(
    state: &str,
    income: f64,
    filing_status: FilingStatus,
    year: i32,
) -> f64 {
    let deduction_amount = crate::get_deductions (state, year, &filing_status, income).standard_deduction;
    let taxable_income = (income - deduction_amount).max (0.0);
    
    println!("DEBUG: Deduction Amount: ${:.2}", deduction_amount);
    println!("DEBUG: Taxable Income after deduction: ${:.2}", taxable_income);

    let brackets = crate::get_tax_brackets_impl::get_tax_brackets(state, year, &filing_status);
    TaxBrackets::new(brackets).taxes(taxable_income)
}

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

    "ok" => { // Oklahoma
        2024 => {
            FilingStatus::Single => deductions::ok_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ok_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ok_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ok_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ok_head_of_household_deduction_2024,
        },
    },

    "or" => { // Oregon
        2024 => {
            FilingStatus::Single => deductions::or_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::or_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::or_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::or_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::or_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::or_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::or_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::or_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::or_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::or_head_of_household_deduction_2025,
        },
    },

    "pa" => { // Pennsylvania
        2024 => {
            FilingStatus::Single => deductions::pa_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::pa_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::pa_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::pa_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::pa_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::pa_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::pa_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::pa_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::pa_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::pa_head_of_household_deduction_2025,
        },
    },

    "ri" => { // Rhode Island
        2024 => {
            FilingStatus::Single => deductions::ri_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::ri_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::ri_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ri_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::ri_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::ri_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::ri_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::ri_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::ri_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::ri_head_of_household_deduction_2025,
        },
    },

    "sc" => { // South Carolina
        2024 => {
            FilingStatus::Single => deductions::sc_single_deduction_2024,
            FilingStatus::MarriedFilingSeparately => deductions::sc_married_filing_separately_deduction_2024,
            FilingStatus::MarriedFilingJointly => deductions::sc_married_filing_jointly_deduction_2024,
            FilingStatus::QualifyingSurvivingSpouse => deductions::sc_qualifying_surviving_spouse_deduction_2024,
            FilingStatus::HeadOfHousehold => deductions::sc_head_of_household_deduction_2024,
        },
        2025 => {
            FilingStatus::Single => deductions::sc_single_deduction_2025,
            FilingStatus::MarriedFilingSeparately => deductions::sc_married_filing_separately_deduction_2025,
            FilingStatus::MarriedFilingJointly => deductions::sc_married_filing_jointly_deduction_2025,
            FilingStatus::QualifyingSurvivingSpouse => deductions::sc_qualifying_surviving_spouse_deduction_2025,
            FilingStatus::HeadOfHousehold => deductions::sc_head_of_household_deduction_2025,
        },
    },
);

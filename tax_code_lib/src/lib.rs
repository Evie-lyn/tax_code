pub mod tax_bracket; 
pub mod deductions;  
pub mod income_based_deduction; 
pub mod brackets;
pub mod get_tax_brackets;
pub mod get_deductions;
pub mod get_taxable_social_security;
pub mod capital_gains;
pub mod federal_capital_gains_tax;
pub mod federal_income_tax;
pub mod get_tax_brackets_class;
pub mod federal_fica_taxes;

use crate::{capital_gains::CapitalGainsResult, tax_bracket::TaxBrackets};


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

#[derive(Debug, PartialEq, Clone)]
pub struct Taxes {
    pub federal_income_tax: f64,
    pub federal_capital_gains_tax: f64,
    pub state_income_tax: f64,
    pub state_capital_gains_tax: f64,
    pub social_security_tax: f64,
    pub fica_tax: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SeparateTaxes {
    pub primary: Taxes,
    pub secondary: Taxes,
}


#[derive(Debug, PartialEq, Clone)]
pub enum TaxResult {
    Joint(Taxes),
    Separate(SeparateTaxes),
}

fn calculate_individual_taxes(
    state: &str,
    income: f64,
    capital_gains: f64,
    social_security_income: f64,
    filing_status: FilingStatus,
    year: i32,
    age: i32,
) -> Taxes {
    let state_deduction_amount = crate::get_deductions::get_deductions(state, year, &filing_status, income).standard_deduction;
    let state_taxable_income = (income - state_deduction_amount).max(0.0);

    let state_taxable_social_security = crate::get_taxable_social_security::get_taxable_social_security(state, year, age, &filing_status, income + capital_gains, social_security_income);
    let federal_taxable_social_security = crate::federal_income_tax::FederalIncomeTaxCalculator::load().unwrap().calculate(year, &filing_status, income + capital_gains);

    let state_brackets = crate::get_tax_brackets_class::StateIncomeTaxBrackets::load().unwrap().get(state, year, &filing_status).unwrap();

    let state_capital_gains = crate::capital_gains::CapitalGainsCalculator::load().unwrap().calculate(state, year, &filing_status, capital_gains, true, None);
    let federal_capital_gains = crate::federal_capital_gains_tax::FederalCapitalGainsCalculator::load().unwrap().calculate(year, &filing_status, capital_gains);

    let mut state_cap_gains_tax = 0.0;
    let mut federal_cap_gains_tax = 0.0;
    let mut state_gains_to_be_taxed = 0.0;
    let mut federal_gains_to_be_taxed = 0.0;

    match state_capital_gains {
        CapitalGainsResult::Taxes(tax) => state_cap_gains_tax = tax,
        CapitalGainsResult::ToBeTaxed(tax) => state_gains_to_be_taxed = tax,
    }

    match federal_capital_gains {
        CapitalGainsResult::Taxes(tax) => federal_cap_gains_tax = tax,
        CapitalGainsResult::ToBeTaxed(tax) => federal_gains_to_be_taxed = tax,
    }

    let state_taxable_income = (state_taxable_income + state_taxable_social_security + state_gains_to_be_taxed).max(0.0);
    let federal_taxable_income = (income + federal_taxable_social_security + federal_gains_to_be_taxed).max(0.0);

    let state_income_tax = TaxBrackets::new(state_brackets).taxes(state_taxable_income);
    let federal_income_tax = crate::federal_income_tax::FederalIncomeTaxCalculator::load().unwrap().calculate(year, &filing_status, federal_taxable_income);
    let fica_tax = crate::federal_fica_taxes::FederalFicaCalculator::new().calculate(income, &filing_status);

    Taxes {
        federal_income_tax,
        federal_capital_gains_tax: federal_cap_gains_tax,
        state_income_tax,
        state_capital_gains_tax: state_cap_gains_tax,
        social_security_tax: state_taxable_social_security,
        fica_tax,
    }
}

pub fn calculate_income_tax( // for income tax
    state: &str,
    primary_income: f64,
    secondary_income: f64,
    primary_capital_gains: f64,
    secondary_capital_gains: f64,
    primary_social_security_income: f64,
    secondary_social_security_income: f64,
    filing_status: FilingStatus,
    year: i32,
    primary_age: i32,
    secondary_age: i32,
) -> TaxResult {
    // Helper function to calculate taxes for an individual


    match filing_status {
        FilingStatus::MarriedFilingJointly => {
            // For joint filing, combine incomes and calculate as one unit
            let combined_income = primary_income + secondary_income;
            let combined_capital_gains = primary_capital_gains + secondary_capital_gains;
            let combined_social_security = primary_social_security_income + secondary_social_security_income;
            
            // Use primary age for joint calculations (could be adjusted based on requirements)
            let joint_taxes = calculate_individual_taxes(
                state,
                combined_income,
                combined_capital_gains,
                combined_social_security,
                filing_status,
                year,
                primary_age,
            );

            TaxResult::Joint(Taxes {
                federal_income_tax: joint_taxes.federal_income_tax,
                federal_capital_gains_tax: joint_taxes.federal_capital_gains_tax,
                state_income_tax: joint_taxes.state_income_tax,
                state_capital_gains_tax: joint_taxes.state_capital_gains_tax,
                social_security_tax: joint_taxes.social_security_tax,
                fica_tax: joint_taxes.fica_tax,
            })
        }
        FilingStatus::MarriedFilingSeparately => {
            // Calculate taxes separately for each spouse
            let primary_taxes = calculate_individual_taxes(
                state,
                primary_income,
                primary_capital_gains,
                primary_social_security_income,
                filing_status,
                year,
                primary_age,
            );

            let secondary_taxes = calculate_individual_taxes(
                state,
                secondary_income,
                secondary_capital_gains,
                secondary_social_security_income,
                filing_status,
                year,
                secondary_age,
            );

            TaxResult::Separate(SeparateTaxes {
                primary: primary_taxes,
                secondary: secondary_taxes,
            })
        }
        _ => {
            // For single, head of household, or qualifying surviving spouse
            // Only calculate for primary taxpayer
            let primary_taxes = calculate_individual_taxes(
                state,
                primary_income,
                primary_capital_gains,
                primary_social_security_income,
                filing_status,
                year,
                primary_age,
            );

            let empty_taxes = Taxes {
                federal_income_tax: 0.0,
                federal_capital_gains_tax: 0.0,
                state_income_tax: 0.0,
                state_capital_gains_tax: 0.0,
                social_security_tax: 0.0,
                fica_tax: 0.0,
            };

            TaxResult::Separate(SeparateTaxes {
                primary: primary_taxes,
                secondary: empty_taxes,
            })
        }
    }
}
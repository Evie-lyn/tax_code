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

pub struct Taxes {
    pub federal_income_tax: f64,
    pub federal_capital_gains_tax: f64,
    pub state_income_tax: f64,
    pub state_capital_gains_tax: f64,
    pub social_security_tax: f64,
}
pub fn calculate_income_tax( // for income tax
    state: &str,
    income: f64,
    capital_gains: f64,
    social_security_income: f64,
    filing_status: FilingStatus,
    year: i32,
    age: i32,
) -> Taxes {
    let state_deduction_amount = crate::get_deductions::get_deductions (state, year, &filing_status, income).standard_deduction;
    let state_taxable_income = (income - state_deduction_amount).max (0.0);

    let state_taxable_social_security = crate::get_taxable_social_security::get_taxable_social_security(state, year, age, &filing_status, income+capital_gains, social_security_income);
    let federal_taxable_social_security = crate::federal_income_tax::FederalIncomeTaxCalculator::load().unwrap().calculate(year, &filing_status, income+capital_gains);

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

    Taxes {
        federal_income_tax: federal_income_tax,
        federal_capital_gains_tax: federal_cap_gains_tax,
        state_income_tax: state_income_tax,
        state_capital_gains_tax: state_cap_gains_tax,
        social_security_tax: state_taxable_social_security,
    }
}
pub mod tax_bracket; 
pub mod deductions;  
pub mod income_based_deduction; 
pub mod brackets;
pub mod get_tax_brackets;
pub mod get_deductions;

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
pub fn calculate_income_tax( // for income tax
    state: &str,
    income: f64,
    filing_status: FilingStatus,
    year: i32,
) -> f64 {
    let deduction_amount = crate::get_deductions::get_deductions (state, year, &filing_status, income).standard_deduction;
    let taxable_income = (income - deduction_amount).max (0.0);
    
    println!("DEBUG: Deduction Amount: ${:.2}", deduction_amount);
    println!("DEBUG: Taxable Income after deduction: ${:.2}", taxable_income);

    let brackets = crate::get_tax_brackets::get_tax_brackets(state, year, &filing_status);
    TaxBrackets::new(brackets).taxes(taxable_income)
}
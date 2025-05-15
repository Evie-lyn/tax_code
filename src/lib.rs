mod brackets;
mod tax_bracket;

use brackets::*;
use tax_bracket::TaxBrackets;

//calculates Income Tax based on income and filing status
pub fn calculate_income_tax(state: &str, income: f64, filing_status: &str) -> f64 {
    let state_lower = state.to_lowercase();
    let status_lower = filing_status.to_lowercase();

    match state_lower.as_str() {
        "ca" => {
            let brackets = match status_lower.as_str() {
                "single" | "married filing separately" => TaxBrackets::new(ca_single_tax()),
                "married filing jointly" | "qualifying surviving spouse" => TaxBrackets::new(ca_joint_tax()),
                "head of household" => TaxBrackets::new(ca_headhouse_tax()),
                _ => {
                    eprintln!("Warning: Invalid filing status '{}' for California. Defaulting to single.", filing_status);
                    TaxBrackets::new(ca_single_tax())
                }
            };
            brackets.taxes(income)
        }
        "tx" => {
            let brackets = match status_lower.as_str() {
                "single" | "married filing separately" | "married filing jointly" | "qualifying surviving spouse" | "head of household" => TaxBrackets::new(tx_single_tax()), // Filing status doesn't matter for TX
                _ => TaxBrackets::new(tx_single_tax()),
            };
            brackets.taxes(income)
        }
        "tn" => {
            let brackets = match status_lower.as_str() {
                "single" | "married filing separately" | "married filing jointly" | "qualifying surviving spouse" | "head of household" => TaxBrackets::new(tn_single_tax()), // Filing status doesn't matter for TN
                _ => TaxBrackets::new(tn_single_tax()),
            };
            brackets.taxes(income)
        }
        _ => {
            eprintln!("Error: State '{}' is not currently supported.", state);
            0.0
        }
    }
}

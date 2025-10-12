pub mod tax_bracket; 
pub mod deductions;  
pub mod income_based_deduction; 
pub mod brackets;
pub mod get_tax_brackets;
pub mod get_deductions;
pub mod social_security_calculator;
pub mod capital_gains;
pub mod federal_capital_gains_tax;
pub mod federal_income_tax;
pub mod get_tax_brackets_class;
pub mod federal_fica_taxes;
pub mod state_exemptions;

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

/// Pre-loaded tax calculator that avoids repeated loading of calculator instances
pub struct TaxCalculator {
    federal_income_tax: crate::federal_income_tax::FederalIncomeTaxCalculator,
    state_brackets: crate::get_tax_brackets_class::StateIncomeTaxBrackets,
    capital_gains: crate::capital_gains::CapitalGainsCalculator,
    federal_capital_gains: crate::federal_capital_gains_tax::FederalCapitalGainsCalculator,
    federal_fica: crate::federal_fica_taxes::FederalFicaCalculator,
    social_security: crate::social_security_calculator::SocialSecurityTaxCalculator,
    state_exemptions: crate::state_exemptions::StateExemptionsCalculator,
}

impl TaxCalculator {
    /// Create a new TaxCalculator instance by loading all the necessary calculators
    pub fn new() -> Result<Self, String> {
        let federal_income_tax = crate::federal_income_tax::FederalIncomeTaxCalculator::load()?;
        let state_brackets = crate::get_tax_brackets_class::StateIncomeTaxBrackets::load()?;
        let capital_gains = crate::capital_gains::CapitalGainsCalculator::load()?;
        let federal_capital_gains = crate::federal_capital_gains_tax::FederalCapitalGainsCalculator::load()?;
        let federal_fica = crate::federal_fica_taxes::FederalFicaCalculator::new();
        let social_security = crate::social_security_calculator::SocialSecurityTaxCalculator::load()?;
        let state_exemptions = crate::state_exemptions::StateExemptionsCalculator::load()?;

        Ok(Self {
            federal_income_tax,
            state_brackets,
            capital_gains,
            federal_capital_gains,
            federal_fica,
            social_security,
            state_exemptions,
        })
    }

    /// Calculate how much W2 income is needed to cover expenses after taxes
    /// Uses binary search to find the required gross income
    /// Only considers W2 income - assumes no capital gains or social security income
    pub fn w2_cost_to_cover(
        &self,
        primary_state: &str,
        secondary_state: &str,
        primary_ss_income: f64,
        secondary_ss_income: f64,
        expenses: f64,
        is_primary_expense: bool,
        filing_status: FilingStatus,
        year: i32,
        primary_age: i32,
        secondary_age: i32,
    ) -> (f64, TaxResult) {
        if expenses == 0.0 {
            let tax_result = self.calculate_income_tax(
                primary_state,
                secondary_state,
                0.0,
                0.0,
                0.0, // no capital gains
                0.0, // no capital gains
                primary_ss_income,
                secondary_ss_income,
                filing_status,
                year,
                primary_age,
                secondary_age,
                true,
            );
            return (0.0, tax_result);
        }

        let mut low = 0.0;
        let mut high = 20.0 * expenses; // Start with 20x expenses as upper bound
        let tolerance = 20.0; // Within $20 is acceptable
        
        loop {
            let test_income = (low + high) / 2.0;
            
            // Set up incomes based on which partner the expense belongs to
            let (primary_income, secondary_income) = if is_primary_expense {
                (test_income, 0.0)
            } else {
                (0.0, test_income)
            };

            let ss_income = if filing_status == FilingStatus::MarriedFilingJointly {
                primary_ss_income + secondary_ss_income
            } else {
                if is_primary_expense {
                    primary_ss_income
                } else {
                    secondary_ss_income
                }
            };

            // Calculate taxes with the test income
            let tax_result = self.calculate_income_tax(
                primary_state,
                secondary_state,
                primary_income,
                secondary_income,
                0.0, // no capital gains
                0.0, // no capital gains
                primary_ss_income,
                secondary_ss_income,
                filing_status,
                year,
                primary_age,
                secondary_age,
                true,
            );

            // Calculate total taxes
            let total_tax = match &tax_result {
                TaxResult::Joint(taxes) => {
                    taxes.federal_income_tax +
                    taxes.federal_capital_gains_tax +
                    taxes.state_income_tax +
                    taxes.state_capital_gains_tax +
                    taxes.fica_tax
                }
                TaxResult::Separate(separate_taxes) => {
                    let primary_total = separate_taxes.primary.federal_income_tax +
                        separate_taxes.primary.federal_capital_gains_tax +
                        separate_taxes.primary.state_income_tax +
                        separate_taxes.primary.state_capital_gains_tax +
                        separate_taxes.primary.fica_tax;
                    
                    let secondary_total = separate_taxes.secondary.federal_income_tax +
                        separate_taxes.secondary.federal_capital_gains_tax +
                        separate_taxes.secondary.state_income_tax +
                        separate_taxes.secondary.state_capital_gains_tax +
                        separate_taxes.secondary.fica_tax;
                    
                    primary_total + secondary_total
                }
            };

            // Calculate remaining balance after taxes and expenses
            let balance = test_income + ss_income - total_tax - expenses;
            
            if balance < 0.0 {
                // Need more income
                low = test_income;
            } else if balance > tolerance {
                // Have too much, can reduce income
                high = test_income;
            } else {
                // Found acceptable solution
                return (test_income, tax_result);
            }

            // Prevent infinite loop if bounds get too close
            if high - low < 0.01 {
                return (high, tax_result);
            }
        }
    }

    /// Calculate taxes for an individual using pre-loaded calculators
    pub fn calculate_individual_taxes(
        &self,
        state: &str,
        income: f64,
        capital_gains: f64,
        social_security_income: f64,
        filing_status: FilingStatus,
        skip_fica: bool,
        year: i32,
        age: i32,
    ) -> Taxes {
        let state_deduction_amount = crate::get_deductions::get_deductions(state, year, &filing_status, income).standard_deduction;
        println!("state_deduction_amount: {}", state_deduction_amount);
        let state_exemption_amount = self.state_exemptions.calc_state_exemptions(state, year, &filing_status);
        println!("state_exemption_amount: {}", state_exemption_amount);
        let state_taxable_income = (income - state_deduction_amount - state_exemption_amount).max(0.0);

        let state_taxable_social_security = self.social_security.get_taxable_social_security(state, year, age, &filing_status, income + capital_gains, social_security_income);
        let federal_taxable_social_security = self.social_security.get_taxable_social_security("US", year, age, &filing_status, income + capital_gains, social_security_income);

        let state_brackets = self.state_brackets.get(state, year, &filing_status).unwrap();

        let state_capital_gains = self.capital_gains.calculate(state, year, &filing_status, capital_gains, true, None);
        let federal_capital_gains = self.federal_capital_gains.calculate(year, &filing_status, capital_gains);

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

        println!("state_taxable_income: {}", state_taxable_income);
        let state_income_tax = TaxBrackets::new(state_brackets).taxes(state_taxable_income);
        let federal_income_tax = self.federal_income_tax.calculate(year, &filing_status, federal_taxable_income);
        let fica_tax = if skip_fica {0.0} else {self.federal_fica.calculate(income)};

        Taxes {
            federal_income_tax,
            federal_capital_gains_tax: federal_cap_gains_tax,
            state_income_tax,
            state_capital_gains_tax: state_cap_gains_tax,
            social_security_tax: state_taxable_social_security,
            fica_tax,
        }
    }

    /// Calculate income tax using pre-loaded calculators
    pub fn calculate_income_tax(
        &self,
        primary_state: &str,
        secondary_state: &str,
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
        skip_fica: bool,
    ) -> TaxResult {
        match filing_status {
            FilingStatus::MarriedFilingJointly => {
                // For joint filing, combine incomes and calculate as one unit
                let combined_income = primary_income + secondary_income;
                let combined_capital_gains = primary_capital_gains + secondary_capital_gains;
                let combined_social_security = primary_social_security_income + secondary_social_security_income;
                
                // For joint filing, use primary state for combined calculation
                // Note: In practice, joint filers typically need to file in their state of residence
                let joint_taxes = self.calculate_individual_taxes(
                    primary_state,
                    combined_income,
                    combined_capital_gains,
                    combined_social_security,
                    filing_status,
                    skip_fica,
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
                // Calculate taxes separately for each spouse using their respective states
                let primary_taxes = self.calculate_individual_taxes(
                    primary_state,
                    primary_income,
                    primary_capital_gains,
                    primary_social_security_income,
                    filing_status,
                    skip_fica,
                    year,
                    primary_age,
                );

                let secondary_taxes = self.calculate_individual_taxes(
                    secondary_state,
                    secondary_income,
                    secondary_capital_gains,
                    secondary_social_security_income,
                    filing_status,
                    skip_fica,
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
                // Only calculate for primary taxpayer using primary state
                let primary_taxes = self.calculate_individual_taxes(
                    primary_state,
                    primary_income,
                    primary_capital_gains,
                    primary_social_security_income,
                    filing_status,
                    skip_fica,
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

    /// Calculate FICA taxes for individual incomes
    pub fn calculate_fica_taxes(
        &self,
        primary_income: f64,
        secondary_income: f64,
        _filing_status: FilingStatus,
    ) -> (f64, f64) {
        let primary_fica_tax = self.federal_fica.calculate(primary_income);
        let secondary_fica_tax = self.federal_fica.calculate(secondary_income);
        (primary_fica_tax, secondary_fica_tax)
    }
}

// Deprecated: Legacy function wrapper - kept for backward compatibility
// Use TaxCalculator::new().unwrap().calculate_income_tax() instead
#[deprecated(note = "Use TaxCalculator::new().unwrap().calculate_income_tax() instead")]
pub fn calculate_income_tax(
    primary_state: &str,
    secondary_state: &str,
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
    let calculator = TaxCalculator::new().expect("Failed to initialize tax calculator");
    calculator.calculate_income_tax(
        primary_state,
        secondary_state,
        primary_income,
        secondary_income,
        primary_capital_gains,
        secondary_capital_gains,
        primary_social_security_income,
        secondary_social_security_income,
        filing_status,
        year,
        primary_age,
        secondary_age,
        false,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_w2_cost_to_cover() {
        let calculator = TaxCalculator::new().expect("Failed to create tax calculator");
        
        // Test with primary partner expenses
        let expenses = 50000.0;
        let (required_income, tax_result) = calculator.w2_cost_to_cover(
            "CA", // primary state
            "WA", // secondary state (different state to test functionality)
            0.0,
            0.0,
            expenses,
            true, // is_primary_expense
            FilingStatus::Single,
            2024,
            35, // primary_age
            35, // secondary_age
        );

        // Verify that the required income minus taxes equals approximately the expenses
        let total_tax = match tax_result {
            TaxResult::Joint(taxes) => {
                taxes.federal_income_tax +
                taxes.federal_capital_gains_tax +
                taxes.state_income_tax +
                taxes.state_capital_gains_tax +
                taxes.fica_tax
            }
            TaxResult::Separate(separate_taxes) => {
                separate_taxes.primary.federal_income_tax +
                separate_taxes.primary.federal_capital_gains_tax +
                separate_taxes.primary.state_income_tax +
                separate_taxes.primary.state_capital_gains_tax +
                separate_taxes.primary.fica_tax
            }
        };

        let net_income = required_income - total_tax;
        
        // Should be within $20 tolerance of the target expenses
        assert!((net_income - expenses).abs() <= 20.0, 
            "Net income {} should be within $20 of expenses {}", net_income, expenses);
        
        // Required income should be greater than expenses (since we need to pay taxes)
        assert!(required_income > expenses, 
            "Required income {} should be greater than expenses {}", required_income, expenses);
    }

    #[test]
    fn test_w2_cost_to_cover_zero_expenses() {
        let calculator = TaxCalculator::new().expect("Failed to create tax calculator");
        
        let (required_income, _) = calculator.w2_cost_to_cover(
            "CA", // primary state
            "WA", // secondary state
            0.0,
            0.0,
            0.0, // zero expenses
            true,
            FilingStatus::Single,
            2024,
            35,
            35,
        );

        assert_eq!(required_income, 0.0, "Zero expenses should require zero income");
    }

    #[test]
    fn test_separate_states_married_filing_separately() {
        let calculator = TaxCalculator::new().expect("Failed to create tax calculator");
        
        // Test with married filing separately where partners live in different states
        let expenses = 30000.0;
        let (required_income, tax_result) = calculator.w2_cost_to_cover(
            "CA", // primary state (high tax state)
            "WA", // secondary state (no income tax state)
            0.0,
            0.0,
            expenses,
            false, // secondary partner's expense (living in WA)
            FilingStatus::MarriedFilingSeparately,
            2024,
            35,
            35,
        );

        // Verify tax calculation worked and we get separate tax results
        match tax_result {
            TaxResult::Separate(separate_taxes) => {
                // Primary should have no income/taxes since is_primary_expense is false
                assert_eq!(separate_taxes.primary.state_income_tax, 0.0, 
                    "Primary partner should have no state income tax with zero income");
                
                // Secondary should have some income and federal taxes but no state taxes (WA has no income tax)
                assert_eq!(separate_taxes.secondary.state_income_tax, 0.0, 
                    "Secondary partner in WA should have no state income tax");
                assert!(separate_taxes.secondary.federal_income_tax > 0.0 || 
                       separate_taxes.secondary.fica_tax > 0.0, 
                    "Secondary partner should have some federal taxes");
            }
            TaxResult::Joint(_) => {
                panic!("Expected separate tax results for MarriedFilingSeparately");
            }
        }

        // Verify the income calculation is reasonable
        assert!(required_income > expenses, 
            "Required income should be greater than expenses due to taxes");
        assert!(required_income < expenses * 2.0, 
            "Required income should be reasonable (less than 2x expenses)");
    }
}
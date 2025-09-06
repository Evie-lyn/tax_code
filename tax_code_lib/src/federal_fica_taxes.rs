use crate::FilingStatus;

#[derive(Debug, Clone)]
pub struct IncomeTaxBracket {
    pub min: f64,
    pub max: f64,
    pub rate: f64,
}

#[derive(Debug, Clone)]
pub struct IncomeTaxTable {
    pub state: String,
    pub table: Vec<IncomeTaxBracket>,
}

pub struct FederalFicaCalculator {
    pub fica_brackets: IncomeTaxTable,
}

impl FederalFicaCalculator {
    pub fn new() -> Self {
        let fica_brackets = IncomeTaxTable {
            state: "FICA".to_string(),
            table: vec![
                IncomeTaxBracket {
                    min: 0.0,
                    max: 176100.0,
                    rate: 0.0765,
                },
                IncomeTaxBracket {
                    min: 176100.0,
                    max: 250000.0,
                    rate: 0.0145,
                },
                IncomeTaxBracket {
                    min: 250000.0,
                    max: f64::INFINITY,
                    rate: 0.0145 + 0.009,
                },
            ]
        };

        FederalFicaCalculator { fica_brackets }
    }

    pub fn calculate(&self, earned_income: f64, _filing_status: &FilingStatus) -> f64 {
        self.calculate_tax_from_brackets(earned_income, &self.fica_brackets.table)
    }

    fn calculate_tax_from_brackets(&self, income: f64, brackets: &Vec<IncomeTaxBracket>) -> f64 {
        let mut tax = 0.0;

        for bracket in brackets {
            let taxable_in_bracket = if income > bracket.max {
                bracket.max - bracket.min
            } else if income > bracket.min {
                income - bracket.min
            } else {
                0.0
            };

            if taxable_in_bracket > 0.0 {
                tax += taxable_in_bracket * bracket.rate;
            }

            if income <= bracket.max {
                break;
            }
        }

        tax
    }
} 
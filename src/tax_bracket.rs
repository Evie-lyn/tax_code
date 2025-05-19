use crate::brackets::Bracket;
pub struct TaxBrackets {
    brackets: Vec<Bracket>,
}

impl TaxBrackets {
    //panics if income limits in brackets are not monotonically increasing
    pub fn new(brackets: Vec<Bracket>) -> Self {
        for i in 1..brackets.len() {
            if brackets[i].0 <= brackets[i - 1].0 {
                panic!("Tax bracket income limits must me increasing monotonically");
            }
        }
        TaxBrackets { brackets }
    }

    pub fn taxes(&self, income: f64) -> f64 {
        let mut tax = 0.0;
        let mut previous_income = 0.0;

        for bracket in &self.brackets {
            let upper_bound = bracket.0;
            let rate = bracket.1;

            let taxable_in_bracket = if income > upper_bound {
                upper_bound - previous_income
            } else {
                income - previous_income
            };

            if taxable_in_bracket > 0.0 {
                tax += taxable_in_bracket * rate;
            }
            if income <= upper_bound {
                break;
            }
            previous_income = upper_bound;
        }
        tax
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brackets::Bracket;

    #[test]
    fn test_monotonic_increase() {
        let ok_brackets = vec![
            Bracket(10.0, 0.1),
            Bracket(20.0, 0.2),
            Bracket(f64::INFINITY, 0.3),
        ];
        TaxBrackets::new(ok_brackets);

        let bad_brackets = vec![Bracket(20.0, 0.1), Bracket(10.0, 0.2)];
        let result = std::panic::catch_unwind(|| TaxBrackets::new(bad_brackets));
        assert!(result.is_err());
    }

    #[test]
    fn test_low_income_single_bracket() {
        let brackets = vec![Bracket(10000.0, 0.01), Bracket(f64::INFINITY, 0.02)];
        let tax_brackets = TaxBrackets::new(brackets);
        let income = 5000.0;
        let expected_tax = 5000.0 * 0.01;
        let actual_tax = tax_brackets.taxes(income);
        let epsilon = 0.0001;
        assert!((actual_tax - expected_tax).abs() < epsilon);
    }

    #[test]
    fn test_income_exactly_at_bracket_boundary() {
        let brackets = vec![
            Bracket(10000.0, 0.01),
            Bracket(30000.0, 0.02),
            Bracket(f64::INFINITY, 0.03),
        ];
        let tax_brackets = TaxBrackets::new(brackets);
        let income = 10000.0;
        let expected_tax = 10000.0 * 0.01;
        let actual_tax = tax_brackets.taxes(income);
        let epsilon = 0.0001;
        assert!((actual_tax - expected_tax).abs() < epsilon);
    }

    #[test]
    fn test_mid_income_multiple_brackets() {
        let brackets = vec![
            Bracket(10000.0, 0.01),
            Bracket(30000.0, 0.02),
            Bracket(f64::INFINITY, 0.03),
        ];
        let tax_brackets = TaxBrackets::new(brackets);
        let income = 20000.0;
        let expected_tax = (10000.0 * 0.01) + ((20000.0 - 10000.0) * 0.02);
        let actual_tax = tax_brackets.taxes(income);
        let epsilon = 0.0001;
        assert!((actual_tax - expected_tax).abs() < epsilon);
    }

    #[test]
    fn test_high_income_hits_last_bracket() {
        let brackets = vec![
            Bracket(10000.0, 0.01),
            Bracket(30000.0, 0.02),
            Bracket(f64::INFINITY, 0.03),
        ];
        let tax_brackets = TaxBrackets::new(brackets);
        let income = 50000.0;
        let expected_tax =
            (10000.0 * 0.01) + ((30000.0 - 10000.0) * 0.02) + ((50000.0 - 30000.0) * 0.03);
        let actual_tax = tax_brackets.taxes(income);
        let epsilon = 0.0001;
        assert!((actual_tax - expected_tax).abs() < epsilon);
    }

    #[test]
    fn test_income_below_first_bracket() {
        let brackets = vec![Bracket(10000.0, 0.01), Bracket(f64::INFINITY, 0.02)];
        let tax_brackets = TaxBrackets::new(brackets);
        let income = 1000.0;
        let expected_tax = 1000.0 * 0.01;
        let actual_tax = tax_brackets.taxes(income);
        let epsilon = 0.0001;
        assert!((actual_tax - expected_tax).abs() < epsilon);
    }

    #[test]
    fn test_income_zero() {
        let brackets = vec![Bracket(10000.0, 0.01), Bracket(f64::INFINITY, 0.02)];
        let tax_brackets = TaxBrackets::new(brackets);
        let income = 0.0;
        let expected_tax = 0.0;
        let actual_tax = tax_brackets.taxes(income);
        let epsilon = 0.0001;
        assert!((actual_tax - expected_tax).abs() < epsilon);
    }

    #[test]
    fn test_brackets_with_zero_rate() {
        let brackets = vec![Bracket(10000.0, 0.0), Bracket(f64::INFINITY, 0.05)];
        let tax_brackets = TaxBrackets::new(brackets);
        let income = 15000.0;
        let expected_tax = (10000.0 * 0.0) + ((15000.0 - 10000.0) * 0.05);
        let actual_tax = tax_brackets.taxes(income);
        let epsilon = 0.0001;
        assert!((actual_tax - expected_tax).abs() < epsilon);
    }
}

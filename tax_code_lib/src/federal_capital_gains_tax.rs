use std::collections::HashMap;
use serde::Deserialize;

use crate::FilingStatus;
use crate::capital_gains::CapitalGainsResult;

const FEDERAL_CAPITAL_GAINS_JSON: &str = include_str!("federal_capital_gains_taxes.json");

#[derive(Debug, Deserialize, Clone)]
struct JsonBracket {
    rate: f64,
    upper_bound: Option<f64>,
}

type CountryYearStatusBrackets = HashMap<String, HashMap<String, HashMap<String, Vec<JsonBracket>>>>;

pub struct FederalCapitalGainsCalculator {
    // year -> filing_status -> brackets
    data: HashMap<String, HashMap<String, Vec<JsonBracket>>>,
}

impl FederalCapitalGainsCalculator {
    pub fn load() -> Result<Self, String> {
        let parsed: CountryYearStatusBrackets = serde_json::from_str(FEDERAL_CAPITAL_GAINS_JSON)
            .map_err(|e| format!("Failed to parse federal_capital_gains_taxes.json: {}", e))?;
        let us = parsed
            .get("US")
            .ok_or_else(|| "Missing 'US' key in federal_capital_gains_taxes.json".to_string())?
            .clone();
        Ok(Self { data: us })
    }

    fn status_key(filing_status: &FilingStatus) -> &'static str {
        match filing_status {
            FilingStatus::Single => "Single",
            FilingStatus::MarriedFilingSeparately => "MarriedFilingSeparately",
            FilingStatus::MarriedFilingJointly => "MarriedFilingJointly",
            FilingStatus::QualifyingSurvivingSpouse => "QualifyingSurvivingSpouse",
            FilingStatus::HeadOfHousehold => "HeadOfHousehold",
        }
    }

    pub fn calculate(&self, year: i32, filing_status: &FilingStatus, capital_gains: f64) -> CapitalGainsResult {
        let gains = capital_gains.max(0.0);
        if gains == 0.0 { return CapitalGainsResult::Taxes(0.0); }

        let year_key = year.to_string();
        let Some(status_map) = self.data.get(&year_key) else {
            // If we have no data for the year, treat gains as ordinary income
            return CapitalGainsResult::ToBeTaxed(gains);
        };
        let status_key = Self::status_key(filing_status);
        let Some(brackets) = status_map.get(status_key) else {
            return CapitalGainsResult::ToBeTaxed(gains);
        };

        let tax = compute_tax_from_json_brackets(gains, brackets);
        CapitalGainsResult::Taxes(tax)
    }
}

fn compute_tax_from_json_brackets(income: f64, brackets: &Vec<JsonBracket>) -> f64 {
    let mut tax = 0.0;
    let mut previous_upper = 0.0;
    for b in brackets {
        let upper = b.upper_bound.unwrap_or(f64::INFINITY);
        let taxable_in_bracket = if income > upper {
            upper - previous_upper
        } else {
            income - previous_upper
        };
        if taxable_in_bracket > 0.0 {
            tax += taxable_in_bracket * b.rate;
        }
        if income <= upper {
            break;
        }
        previous_upper = upper;
    }
    tax
}

use std::collections::HashMap;
use serde::Deserialize;

use crate::FilingStatus;

const FEDERAL_INCOME_TAX_JSON: &str = include_str!("federal_income_taxes.json");

#[derive(Debug, Deserialize, Clone)]
struct JsonBracket {
	rate: f64,
	upper_bound: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
struct FederalIncomeTaxData {
	// year -> filing_status -> amount
	deductions: HashMap<String, HashMap<String, f64>>,
	// year -> filing_status -> brackets
	brackets: HashMap<String, HashMap<String, Vec<JsonBracket>>>,
}

pub struct FederalIncomeTaxCalculator {
	data: FederalIncomeTaxData,
}

impl FederalIncomeTaxCalculator {
	pub fn load() -> Result<Self, String> {
		let parsed: FederalIncomeTaxData = serde_json::from_str(FEDERAL_INCOME_TAX_JSON)
			.map_err(|e| format!("Failed to parse federal_income_taxes.json: {}", e))?;
		Ok(Self { data: parsed })
	}

	fn status_key(filing_status: &FilingStatus) -> &'static str {
		match filing_status {
			FilingStatus::Single => "Single",
			FilingStatus::MarriedFilingSeparately => "Single", // fallback when separate not provided
			FilingStatus::MarriedFilingJointly => "MarriedFilingJointly",
			FilingStatus::QualifyingSurvivingSpouse => "MarriedFilingJointly", // closest match
			FilingStatus::HeadOfHousehold => "Single", // fallback when HOH not provided
		}
	}

	fn get_deduction(&self, year: i32, filing_status: &FilingStatus) -> f64 {
		let year_key = year.to_string();
		let status_key = Self::status_key(filing_status);
		self
			.data
			.deductions
			.get(&year_key)
			.and_then(|m| m.get(status_key))
			.copied()
			.unwrap_or(0.0)
	}

	fn get_brackets(&self, year: i32, filing_status: &FilingStatus) -> Option<&Vec<JsonBracket>> {
		let year_key = year.to_string();
		let status_key = Self::status_key(filing_status);
		self
			.data
			.brackets
			.get(&year_key)
			.and_then(|m| m.get(status_key))
	}

	pub fn calculate(&self, year: i32, filing_status: &FilingStatus, income: f64) -> f64 {
		let income_non_negative = income.max(0.0);
		let deduction = self.get_deduction(year, filing_status);
		let taxable_income = (income_non_negative - deduction).max(0.0);

		let Some(brackets) = self.get_brackets(year, filing_status) else {
			return 0.0;
		};
		compute_tax_from_json_brackets(taxable_income, brackets)
	}
}

fn compute_tax_from_json_brackets(income: f64, brackets: &Vec<JsonBracket>) -> f64 {
	let mut tax = 0.0;
	let mut previous_upper = 0.0;
	for b in brackets {
		let upper = b.upper_bound.unwrap_or(f64::INFINITY);
		let taxable_in_bracket = if income > upper { upper - previous_upper } else { income - previous_upper };
		if taxable_in_bracket > 0.0 {
			tax += taxable_in_bracket * b.rate;
		}
		if income <= upper { break; }
		previous_upper = upper;
	}
	tax
} 
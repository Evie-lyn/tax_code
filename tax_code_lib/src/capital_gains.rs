use std::collections::HashMap;
use serde::Deserialize;

use crate::FilingStatus;

const CAPITAL_GAINS_JSON: &str = include_str!("states_with_capital_gains.json");

#[derive(Debug, Deserialize, Clone)]
struct JsonBracket {
    rate: f64,
    upper_bound: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
struct StateCapitalGainsPolicy {
    #[serde(default)]
    long_term_discount: Option<f64>,
    #[serde(default)]
    minimum_discount: Option<f64>,
    #[serde(default)]
    long_term_max_rate: Option<f64>,
    #[serde(default)]
    requires_3yr_hold: Option<bool>,
    #[serde(default)]
    brackets: Option<HashMap<String, Vec<JsonBracket>>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CapitalGainsResult {
    Taxes(f64),
    ToBeTaxed(f64),
}

pub struct CapitalGainsCalculator {
    // year -> state -> policy (None when the JSON had null)
    data: HashMap<String, HashMap<String, Option<StateCapitalGainsPolicy>>>,
}

impl CapitalGainsCalculator {
    pub fn load() -> Result<Self, String> {
        let parsed: HashMap<String, HashMap<String, Option<StateCapitalGainsPolicy>>> = serde_json::from_str(CAPITAL_GAINS_JSON)
            .map_err(|e| format!("Failed to parse states_with_capital_gains.json: {}", e))?;
        Ok(Self { data: parsed })
    }

    fn get_policy(&self, state: &str, year: i32) -> Option<Option<StateCapitalGainsPolicy>> {
        let state_key = state.to_uppercase();
        // Try exact year first
        if let Some(policy) = self.data
            .get(&year.to_string())
            .and_then(|m| m.get(&state_key).cloned())
        {
            return Some(policy);
        }

        // Fallback: use the most recent year available for this state
        let mut latest: Option<(i32, Option<StateCapitalGainsPolicy>)> = None;
        for (year_str, state_map) in &self.data {
            if let Ok(y) = year_str.parse::<i32>() {
                if let Some(policy) = state_map.get(&state_key).cloned() {
                    match latest {
                        Some((ly, _)) if y <= ly => {}
                        _ => {
                            latest = Some((y, policy));
                        }
                    }
                }
            }
        }
        latest.map(|(_, policy)| policy)
    }

    pub fn calculate(
        &self,
        state: &str,
        year: i32,
        filing_status: &FilingStatus,
        capital_gains: f64,
        is_long_term: bool,
        years_held: Option<i32>,
    ) -> CapitalGainsResult {
        let gains = capital_gains.max(0.0);
        if gains == 0.0 {
            return CapitalGainsResult::Taxes(0.0);
        }

        match self.get_policy(state, year) {
            None => {
                // No entry for this year/state: treat as ordinary income
                CapitalGainsResult::ToBeTaxed(gains)
            }
            Some(None) => {
                // Explicit null means no state capital gains tax
                CapitalGainsResult::Taxes(0.0)
            }
            Some(Some(policy)) => {
                // Handle explicit brackets (e.g., MT)
                if let Some(brackets_by_status) = &policy.brackets {
                    let status_key = match filing_status {
                        FilingStatus::Single => "Single",
                        FilingStatus::MarriedFilingSeparately => "MarriedFilingSeparately",
                        FilingStatus::MarriedFilingJointly => "MarriedFilingJointly",
                        FilingStatus::QualifyingSurvivingSpouse => "QualifyingSurvivingSpouse",
                        FilingStatus::HeadOfHousehold => "HeadOfHousehold",
                    };
                    if let Some(brackets) = brackets_by_status.get(status_key) {
                        let tax = compute_tax_from_json_brackets(gains, brackets);
                        return CapitalGainsResult::Taxes(tax);
                    }
                    // If no brackets for this filing status, fall back to ordinary income
                    return CapitalGainsResult::ToBeTaxed(gains);
                }

                // Handle states with a capped long-term maximum rate (e.g., HI)
                if let Some(max_rate) = policy.long_term_max_rate {
                    if is_long_term {
                        let capped_tax = gains * max_rate;
                        return CapitalGainsResult::Taxes(capped_tax);
                    } else {
                        return CapitalGainsResult::ToBeTaxed(gains);
                    }
                }

                // Handle percentage discount + minimum discount
                let long_term_discount = policy.long_term_discount.unwrap_or(0.0);
                let minimum_discount = policy.minimum_discount.unwrap_or(0.0);
                let requires_3yr_hold = policy.requires_3yr_hold.unwrap_or(false);

                if long_term_discount == 0.0 && minimum_discount == 0.0 {
                    // Treated as ordinary income
                    return CapitalGainsResult::ToBeTaxed(gains);
                }

                if !is_long_term {
                    // Short-term gains do not get long-term discount
                    return CapitalGainsResult::ToBeTaxed(gains);
                }

                if requires_3yr_hold {
                    match years_held {
                        Some(y) if y >= 3 => {}
                        _ => {
                            // Requirement not met, treat as ordinary income
                            return CapitalGainsResult::ToBeTaxed(gains);
                        }
                    }
                }

                let mut discount_amount = gains * long_term_discount;
                let min_applicable = minimum_discount.min(gains);
                if discount_amount < min_applicable {
                    discount_amount = min_applicable;
                }
                let taxable_after_discount = (gains - discount_amount).max(0.0);
                CapitalGainsResult::ToBeTaxed(taxable_after_discount)
            }
        }
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
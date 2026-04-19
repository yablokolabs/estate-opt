use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};
use thiserror::Error;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StrategyMode {
    Income,
    Resale,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Property {
    pub id: String,
    pub city: String,
    pub locality: String,
    pub price: f64,
    pub expected_rent: f64,
    pub cap_rate: f64,
    pub expected_sale_price: f64,
    pub appreciation_score: f64,
    pub vacancy_risk: f64,
    pub repair_cost: f64,
    pub liquidity_score: f64,
    pub holding_horizon_months: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HardConstraints {
    pub max_budget: Option<f64>,
    pub city: Option<String>,
    pub min_liquidity_score: Option<f64>,
    pub max_vacancy_risk: Option<f64>,
}

impl HardConstraints {
    pub fn allows(&self, property: &Property) -> bool {
        self.max_budget.is_none_or(|b| property.price <= b)
            && self.city.as_ref().is_none_or(|c| property.city == *c)
            && self
                .min_liquidity_score
                .is_none_or(|s| property.liquidity_score >= s)
            && self
                .max_vacancy_risk
                .is_none_or(|r| property.vacancy_risk <= r)
    }
}

#[derive(Debug, Error)]
pub enum PropertyLoadError {
    #[error("failed to open CSV: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse CSV: {0}")]
    Csv(#[from] csv::Error),
}

pub fn load_properties_csv(path: impl AsRef<Path>) -> Result<Vec<Property>, PropertyLoadError> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_reader(file);
    let mut rows = Vec::new();
    for record in reader.deserialize() {
        rows.push(record?);
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_constraints_filter_properties() {
        let property = Property {
            id: "la-001".to_string(),
            city: "los-angeles".to_string(),
            locality: "silver-lake".to_string(),
            price: 1_280_000.0,
            expected_rent: 72_000.0,
            cap_rate: 0.068,
            expected_sale_price: 1_520_000.0,
            appreciation_score: 0.74,
            vacancy_risk: 0.07,
            repair_cost: 0.10,
            liquidity_score: 0.81,
            holding_horizon_months: 60,
        };

        let allowed = HardConstraints {
            max_budget: Some(1_500_000.0),
            city: Some("los-angeles".to_string()),
            min_liquidity_score: Some(0.8),
            max_vacancy_risk: Some(0.1),
        };
        assert!(allowed.allows(&property));

        let blocked = HardConstraints {
            max_budget: Some(1_000_000.0),
            city: None,
            min_liquidity_score: None,
            max_vacancy_risk: None,
        };
        assert!(!blocked.allows(&property));
    }
}

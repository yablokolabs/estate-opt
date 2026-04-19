use estate_opt_core::Property;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreWeights {
    pub yield_weight: f64,
    pub liquidity_weight: f64,
    pub risk_weight: f64,
    pub repair_cost_weight: f64,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            yield_weight: 1.0,
            liquidity_weight: 0.7,
            risk_weight: 0.8,
            repair_cost_weight: 0.4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreBreakdown {
    pub total: f64,
    pub yield_component: f64,
    pub liquidity_component: f64,
    pub risk_penalty: f64,
    pub repair_penalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Explanation {
    pub summary: String,
    pub strengths: Vec<String>,
    pub tradeoffs: Vec<String>,
}

pub fn score_property(property: &Property, weights: &ScoreWeights) -> ScoreBreakdown {
    let yield_component = property.cap_rate * weights.yield_weight;
    let liquidity_component = property.liquidity_score * weights.liquidity_weight;
    let risk_penalty = property.vacancy_risk * weights.risk_weight;
    let repair_penalty = property.repair_cost * weights.repair_cost_weight;
    let total = yield_component + liquidity_component - risk_penalty - repair_penalty;

    ScoreBreakdown {
        total,
        yield_component,
        liquidity_component,
        risk_penalty,
        repair_penalty,
    }
}

pub fn explain_property(property: &Property, breakdown: &ScoreBreakdown) -> Explanation {
    let mut strengths = Vec::new();
    let mut tradeoffs = Vec::new();

    if property.cap_rate >= 0.07 {
        strengths.push("strong cap rate for income-oriented ranking".to_string());
    }
    if property.liquidity_score >= 0.75 {
        strengths.push("above-average liquidity profile".to_string());
    }
    if property.vacancy_risk >= 0.1 {
        tradeoffs.push("vacancy risk may reduce downside resilience".to_string());
    }
    if property.repair_cost >= 0.15 {
        tradeoffs.push("repair cost burden weakens near-term returns".to_string());
    }

    Explanation {
        summary: format!(
            "{} in {} scored {:.3} with yield {:.3}, liquidity {:.3}, risk penalty {:.3}, and repair penalty {:.3}.",
            property.id,
            property.locality,
            breakdown.total,
            breakdown.yield_component,
            breakdown.liquidity_component,
            breakdown.risk_penalty,
            breakdown.repair_penalty
        ),
        strengths,
        tradeoffs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_property() -> Property {
        Property {
            id: "blr-001".to_string(),
            city: "bengaluru".to_string(),
            locality: "indiranagar".to_string(),
            price: 4_800_000.0,
            expected_rent: 180_000.0,
            cap_rate: 0.074,
            vacancy_risk: 0.08,
            repair_cost: 0.12,
            liquidity_score: 0.82,
            holding_horizon_months: 60,
        }
    }

    #[test]
    fn scoring_stays_finite() {
        let property = sample_property();
        let breakdown = score_property(&property, &ScoreWeights::default());
        assert!(breakdown.total.is_finite());
    }

    #[test]
    fn explanation_mentions_strengths() {
        let property = sample_property();
        let breakdown = score_property(&property, &ScoreWeights::default());
        let explanation = explain_property(&property, &breakdown);
        assert!(!explanation.summary.is_empty());
        assert!(!explanation.strengths.is_empty());
    }
}

use estate_opt_core::{Property, StrategyMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreWeights {
    pub yield_weight: f64,
    pub liquidity_weight: f64,
    pub risk_weight: f64,
    pub repair_cost_weight: f64,
    pub appreciation_weight: f64,
    pub resale_spread_weight: f64,
}

impl Default for ScoreWeights {
    fn default() -> Self {
        Self {
            yield_weight: 1.0,
            liquidity_weight: 0.7,
            risk_weight: 0.8,
            repair_cost_weight: 0.4,
            appreciation_weight: 0.9,
            resale_spread_weight: 0.6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoreBreakdown {
    pub total: f64,
    pub strategy_mode: StrategyMode,
    pub yield_component: f64,
    pub liquidity_component: f64,
    pub appreciation_component: f64,
    pub resale_spread_component: f64,
    pub risk_penalty: f64,
    pub repair_penalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Explanation {
    pub summary: String,
    pub strengths: Vec<String>,
    pub tradeoffs: Vec<String>,
}

pub fn score_property(
    property: &Property,
    weights: &ScoreWeights,
    strategy_mode: StrategyMode,
) -> ScoreBreakdown {
    let yield_component = property.cap_rate * weights.yield_weight;
    let liquidity_component = property.liquidity_score * weights.liquidity_weight;
    let appreciation_component = property.appreciation_score * weights.appreciation_weight;
    let resale_spread_component = ((property.expected_sale_price - property.price)
        / property.price.max(1.0))
        * weights.resale_spread_weight;
    let risk_penalty = property.vacancy_risk * weights.risk_weight;
    let repair_penalty = property.repair_cost * weights.repair_cost_weight;

    let total = match strategy_mode {
        StrategyMode::Income => {
            yield_component + liquidity_component - risk_penalty - repair_penalty
        }
        StrategyMode::Resale => {
            appreciation_component + resale_spread_component + liquidity_component
                - risk_penalty
                - repair_penalty
        }
        StrategyMode::Hybrid => {
            yield_component + liquidity_component + appreciation_component + resale_spread_component
                - risk_penalty
                - repair_penalty
        }
    };

    ScoreBreakdown {
        total,
        strategy_mode,
        yield_component,
        liquidity_component,
        appreciation_component,
        resale_spread_component,
        risk_penalty,
        repair_penalty,
    }
}

pub fn explain_property(property: &Property, breakdown: &ScoreBreakdown) -> Explanation {
    let mut strengths = Vec::new();
    let mut tradeoffs = Vec::new();

    if property.cap_rate >= 0.07 {
        strengths.push("strong income profile from cap rate".to_string());
    }
    if property.appreciation_score >= 0.7 {
        strengths.push("strong appreciation outlook for resale scenarios".to_string());
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

    let strategy_label = match breakdown.strategy_mode {
        StrategyMode::Income => "income",
        StrategyMode::Resale => "resale",
        StrategyMode::Hybrid => "hybrid",
    };

    Explanation {
        summary: format!(
            "{} in {} scored {:.3} in {} mode with yield {:.3}, liquidity {:.3}, appreciation {:.3}, resale spread {:.3}, risk penalty {:.3}, and repair penalty {:.3}.",
            property.id,
            property.locality,
            breakdown.total,
            strategy_label,
            breakdown.yield_component,
            breakdown.liquidity_component,
            breakdown.appreciation_component,
            breakdown.resale_spread_component,
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
        }
    }

    #[test]
    fn scoring_stays_finite() {
        let property = sample_property();
        let breakdown = score_property(&property, &ScoreWeights::default(), StrategyMode::Hybrid);
        assert!(breakdown.total.is_finite());
    }

    #[test]
    fn explanation_mentions_strengths() {
        let property = sample_property();
        let breakdown = score_property(&property, &ScoreWeights::default(), StrategyMode::Hybrid);
        let explanation = explain_property(&property, &breakdown);
        assert!(!explanation.summary.is_empty());
        assert!(!explanation.strengths.is_empty());
    }
}

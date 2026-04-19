use estate_opt_core::{HardConstraints, Property, StrategyMode};
use estate_opt_scoring::{
    Explanation, ScoreBreakdown, ScoreWeights, explain_property, score_property,
};
use rand::{Rng, SeedableRng, rngs::StdRng};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RankedProperty {
    pub property: Property,
    pub breakdown: ScoreBreakdown,
    pub explanation: Explanation,
}

pub fn greedy_rank(
    properties: &[Property],
    constraints: &HardConstraints,
    weights: &ScoreWeights,
    strategy_mode: StrategyMode,
) -> Vec<RankedProperty> {
    let mut ranked: Vec<_> = properties
        .iter()
        .filter(|property| constraints.allows(property))
        .map(|property| {
            let breakdown = score_property(property, weights, strategy_mode);
            RankedProperty {
                property: property.clone(),
                explanation: explain_property(property, &breakdown),
                breakdown,
            }
        })
        .collect();

    ranked.sort_by(|a, b| b.breakdown.total.total_cmp(&a.breakdown.total));
    ranked
}

pub fn annealing_rank(
    properties: &[Property],
    constraints: &HardConstraints,
    weights: &ScoreWeights,
    strategy_mode: StrategyMode,
    seed: u64,
    steps: usize,
) -> Vec<RankedProperty> {
    let mut ranked = greedy_rank(properties, constraints, weights, strategy_mode);
    if ranked.len() < 2 {
        return ranked;
    }

    let n = ranked.len();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut temperature = 1.0;
    for _ in 0..steps {
        let i = rng.random_range(0..n);
        let j = rng.random_range(0..n);
        if i == j {
            continue;
        }
        // Position-weighted delta: placing higher-scoring items at lower indices
        // (better positions) increases the objective. A positive delta means the
        // swap improves the ranking.
        let delta = (ranked[j].breakdown.total - ranked[i].breakdown.total)
            * (j as f64 - i as f64);
        if delta > 0.0 || rng.random::<f64>() < (delta / temperature).exp() {
            ranked.swap(i, j);
        }
        temperature *= 0.995;
    }

    ranked
}

pub fn generate_synthetic_properties(count: usize, seed: u64) -> Vec<Property> {
    let mut rng = StdRng::seed_from_u64(seed);
    let cities = ["los-angeles", "new-york", "miami"];
    let localities = ["core", "growth", "suburban", "premium"];

    (0..count)
        .map(|idx| {
            let price = rng.random_range(650_000.0..4_500_000.0);
            Property {
                id: format!("prop-{idx:05}"),
                city: cities[rng.random_range(0..cities.len())].to_string(),
                locality: localities[rng.random_range(0..localities.len())].to_string(),
                price,
                expected_rent: rng.random_range(36_000.0..240_000.0),
                cap_rate: rng.random_range(0.03..0.11),
                expected_sale_price: price * rng.random_range(1.02..1.35),
                appreciation_score: rng.random_range(0.2..0.95),
                vacancy_risk: rng.random_range(0.02..0.18),
                repair_cost: rng.random_range(0.01..0.25),
                liquidity_score: rng.random_range(0.2..0.95),
                holding_horizon_months: rng.random_range(24..121),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_generation_is_reproducible() {
        let a = generate_synthetic_properties(5, 42);
        let b = generate_synthetic_properties(5, 42);
        assert_eq!(a, b);
    }

    #[test]
    fn annealing_produces_valid_ranking() {
        let properties = generate_synthetic_properties(20, 5);
        let constraints = HardConstraints {
            max_budget: None,
            city: None,
            min_liquidity_score: None,
            max_vacancy_risk: None,
        };
        let greedy = greedy_rank(
            &properties,
            &constraints,
            &ScoreWeights::default(),
            StrategyMode::Hybrid,
        );
        let annealed = annealing_rank(
            &properties,
            &constraints,
            &ScoreWeights::default(),
            StrategyMode::Hybrid,
            7,
            5000,
        );
        assert_eq!(annealed.len(), greedy.len());
        for item in &annealed {
            assert!(item.breakdown.total.is_finite());
        }
        // With enough steps, the top of the ranking should converge toward
        // high-scoring items. Verify the top item is in the greedy top-3.
        let greedy_top3: Vec<_> = greedy.iter().take(3).map(|r| &r.property.id).collect();
        assert!(
            greedy_top3.contains(&&annealed[0].property.id),
            "annealing top pick {:?} not in greedy top 3 {:?}",
            annealed[0].property.id,
            greedy_top3
        );
    }
}

use estate_opt_core::{HardConstraints, Property};
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
) -> Vec<RankedProperty> {
    let mut ranked: Vec<_> = properties
        .iter()
        .filter(|property| constraints.allows(property))
        .map(|property| {
            let breakdown = score_property(property, weights);
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
    seed: u64,
    steps: usize,
) -> Vec<RankedProperty> {
    let mut ranked = greedy_rank(properties, constraints, weights);
    if ranked.len() < 2 {
        return ranked;
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let mut temperature = 1.0;
    for _ in 0..steps {
        let i = rng.random_range(0..ranked.len());
        let j = rng.random_range(0..ranked.len());
        if i == j {
            continue;
        }
        let current = ranked[i].breakdown.total + ranked[j].breakdown.total;
        let swapped = ranked[j].breakdown.total + ranked[i].breakdown.total;
        let accept = swapped >= current || rng.random::<f64>() < temperature;
        if accept {
            ranked.swap(i, j);
        }
        temperature *= 0.995;
    }

    ranked.sort_by(|a, b| b.breakdown.total.total_cmp(&a.breakdown.total));
    ranked
}

pub fn generate_synthetic_properties(count: usize, seed: u64) -> Vec<Property> {
    let mut rng = StdRng::seed_from_u64(seed);
    let cities = ["bengaluru", "mumbai", "hyderabad"];
    let localities = ["core", "growth", "suburban", "premium"];

    (0..count)
        .map(|idx| Property {
            id: format!("prop-{idx:05}"),
            city: cities[rng.random_range(0..cities.len())].to_string(),
            locality: localities[rng.random_range(0..localities.len())].to_string(),
            price: rng.random_range(2_000_000.0..15_000_000.0),
            expected_rent: rng.random_range(80_000.0..400_000.0),
            cap_rate: rng.random_range(0.03..0.11),
            vacancy_risk: rng.random_range(0.02..0.18),
            repair_cost: rng.random_range(0.01..0.25),
            liquidity_score: rng.random_range(0.2..0.95),
            holding_horizon_months: rng.random_range(24..121),
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
    fn annealing_keeps_sorted_output_shape() {
        let properties = generate_synthetic_properties(20, 5);
        let constraints = HardConstraints {
            max_budget: None,
            city: None,
            min_liquidity_score: None,
            max_vacancy_risk: None,
        };
        let ranked = annealing_rank(&properties, &constraints, &ScoreWeights::default(), 7, 25);
        assert!(!ranked.is_empty());
        for window in ranked.windows(2) {
            assert!(window[0].breakdown.total >= window[1].breakdown.total);
        }
    }
}

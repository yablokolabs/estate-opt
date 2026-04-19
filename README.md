# estate-opt

Rust-native optimization engine for real estate portfolios and deal decisions using quantum-inspired heuristics, classical solvers, and explainable scoring.

## Status
Early workspace scaffold for a practical open-source optimization engine focused on:
- property ranking under constraints
- portfolio allocation support
- explainable scoring
- solver benchmarking across greedy, annealing, genetic, and experimental QUBO-like paths

## Current capabilities
- canonical property schema
- CSV property loading
- hard-constraint filtering
- weighted scoring model
- explanation summaries for ranked properties
- greedy baseline ranking
- reproducible synthetic dataset generation
- CLI demo ranking flows

## Workspace
- `estate-opt-core`: canonical property schema, constraints, CSV loading
- `estate-opt-scoring`: scoring model, normalization direction, explanation primitives
- `estate-opt-solvers`: greedy baseline and synthetic fixture generation
- `estate-opt-cli`: command-line interface for ranking workflows

## Example commands
```bash
cargo run -p estate-opt-cli -- demo-rank --city bengaluru --budget 5000000 --count 1000
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --city bengaluru --budget 5000000 --top 3
```

More hands-on examples live in `examples/USAGE.md`.

## Principles
- practical first, not hype first
- classical baselines before experimental heuristics
- explainability is a feature, not an afterthought
- domain assumptions must stay explicit and reviewable
- Lean alignment, if added later, should mirror specs and principles rather than claim full floating-point verification

## Near-term roadmap
1. normalization refinements and multi-objective controls
2. annealing solver
3. genetic search solver
4. benchmark harnesses with manual acceptance criteria
5. README variants for open-source, investor, and proptech audiences

## Rendered examples

### Text output
```text
blr-002 | whitefield | score 0.540 | blr-002 in whitefield scored 0.540 with yield 0.071, liquidity 0.553, risk penalty 0.048, and repair penalty 0.036.
blr-001 | indiranagar | score 0.536 | blr-001 in indiranagar scored 0.536 with yield 0.074, liquidity 0.574, risk penalty 0.064, and repair penalty 0.048.
hyd-001 | gachibowli | score 0.515 | hyd-001 in gachibowli scored 0.515 with yield 0.076, liquidity 0.539, risk penalty 0.056, and repair penalty 0.044.
```

### JSON output
```json
[
  {
    "property": {
      "id": "blr-002",
      "city": "bengaluru",
      "locality": "whitefield",
      "price": 5200000.0,
      "expected_rent": 195000.0,
      "cap_rate": 0.071,
      "vacancy_risk": 0.06,
      "repair_cost": 0.09,
      "liquidity_score": 0.79,
      "holding_horizon_months": 48
    },
    "breakdown": {
      "total": 0.5399999999999998,
      "yield_component": 0.071,
      "liquidity_component": 0.5529999999999999,
      "risk_penalty": 0.048,
      "repair_penalty": 0.036
    },
    "explanation": {
      "summary": "blr-002 in whitefield scored 0.540 with yield 0.071, liquidity 0.553, risk penalty 0.048, and repair penalty 0.036.",
      "strengths": [
        "strong cap rate for income-oriented ranking",
        "above-average liquidity profile"
      ],
      "tradeoffs": []
    }
  },
  {
    "property": {
      "id": "blr-001",
      "city": "bengaluru",
      "locality": "indiranagar",
      "price": 4800000.0,
      "expected_rent": 180000.0,
      "cap_rate": 0.074,
      "vacancy_risk": 0.08,
      "repair_cost": 0.12,
      "liquidity_score": 0.82,
      "holding_horizon_months": 60
    },
    "breakdown": {
      "total": 0.5359999999999998,
      "yield_component": 0.074,
      "liquidity_component": 0.574,
      "risk_penalty": 0.064,
      "repair_penalty": 0.048
    },
    "explanation": {
      "summary": "blr-001 in indiranagar scored 0.536 with yield 0.074, liquidity 0.574, risk penalty 0.064, and repair penalty 0.048.",
      "strengths": [
        "strong cap rate for income-oriented ranking",
        "above-average liquidity profile"
      ],
      "tradeoffs": []
    }
  }
]
```

### Markdown report output
```markdown
| id | locality | score | summary |
| --- | --- | ---: | --- |
| blr-002 | whitefield | 0.540 | blr-002 in whitefield scored 0.540 with yield 0.071, liquidity 0.553, risk penalty 0.048, and repair penalty 0.036. |
| blr-001 | indiranagar | 0.536 | blr-001 in indiranagar scored 0.536 with yield 0.074, liquidity 0.574, risk penalty 0.064, and repair penalty 0.048. |
| hyd-001 | gachibowli | 0.515 | hyd-001 in gachibowli scored 0.515 with yield 0.076, liquidity 0.539, risk penalty 0.056, and repair penalty 0.044. |
```

See `examples/USAGE.md` for more command variants and explanations.


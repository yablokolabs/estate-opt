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
cargo run -p estate-opt-cli -- demo-rank --city los-angeles --budget 1500000 --count 1000
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --city los-angeles --budget 1500000 --top 3
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
ny-002 | park-slope | score 0.637 | ny-002 in park-slope scored 0.637 with yield 0.066, liquidity 0.623, risk penalty 0.024, and repair penalty 0.028.
la-002 | santa-monica | score 0.630 | la-002 in santa-monica scored 0.630 with yield 0.070, liquidity 0.616, risk penalty 0.032, and repair penalty 0.024.
ny-001 | astoria | score 0.587 | ny-001 in astoria scored 0.587 with yield 0.071, liquidity 0.588, risk penalty 0.040, and repair penalty 0.032.
```

### JSON output
```json
[
  {
    "property": {
      "id": "ny-002",
      "city": "new-york",
      "locality": "park-slope",
      "price": 1780000.0,
      "expected_rent": 98000.0,
      "cap_rate": 0.066,
      "vacancy_risk": 0.03,
      "repair_cost": 0.07,
      "liquidity_score": 0.89,
      "holding_horizon_months": 84
    },
    "breakdown": {
      "total": 0.637,
      "yield_component": 0.066,
      "liquidity_component": 0.623,
      "risk_penalty": 0.024,
      "repair_penalty": 0.028000000000000004
    },
    "explanation": {
      "summary": "ny-002 in park-slope scored 0.637 with yield 0.066, liquidity 0.623, risk penalty 0.024, and repair penalty 0.028.",
      "strengths": [
        "above-average liquidity profile"
      ],
      "tradeoffs": []
    }
  },
  {
    "property": {
      "id": "la-002",
      "city": "los-angeles",
      "locality": "santa-monica",
      "price": 1650000.0,
      "expected_rent": 96000.0,
      "cap_rate": 0.07,
      "vacancy_risk": 0.04,
      "repair_cost": 0.06,
      "liquidity_score": 0.88,
      "holding_horizon_months": 72
    },
    "breakdown": {
      "total": 0.6299999999999999,
      "yield_component": 0.07,
      "liquidity_component": 0.616,
      "risk_penalty": 0.032,
      "repair_penalty": 0.024
    },
    "explanation": {
      "summary": "la-002 in santa-monica scored 0.630 with yield 0.070, liquidity 0.616, risk penalty 0.032, and repair penalty 0.024.",
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
| ny-002 | park-slope | 0.637 | ny-002 in park-slope scored 0.637 with yield 0.066, liquidity 0.623, risk penalty 0.024, and repair penalty 0.028. |
| la-002 | santa-monica | 0.630 | la-002 in santa-monica scored 0.630 with yield 0.070, liquidity 0.616, risk penalty 0.032, and repair penalty 0.024. |
| ny-001 | astoria | 0.587 | ny-001 in astoria scored 0.587 with yield 0.071, liquidity 0.588, risk penalty 0.040, and repair penalty 0.032. |
```

See `examples/USAGE.md` for more command variants and explanations.

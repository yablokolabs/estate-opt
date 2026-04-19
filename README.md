# estate-opt

Rust-native optimization engine for real estate portfolios and deal decisions using quantum-inspired heuristics, classical solvers, and explainable scoring.

## In one sentence
`estate-opt` helps rank real estate opportunities under practical constraints like budget by combining explainable scoring, classical search baselines, and an experimental quantum-inspired annealing-style heuristic.

## Plain-English explanation
For a novice real estate agent, the simplest way to think about `estate-opt` is:
- start with a buyer or investor budget
- filter out properties that do not fit the hard constraints
- choose whether the goal is income, resale, or a hybrid strategy
- score the remaining options across factors like yield, liquidity, appreciation, vacancy risk, and repair cost
- compare simple ranking methods with more advanced heuristic search
- explain why the top properties ranked above the others

So the product is **not** claiming magical quantum computing. It is a practical decision-support engine that helps surface better property choices in a transparent way.

## Status
Early workspace scaffold for a practical open-source optimization engine focused on:
- property ranking under constraints
- portfolio allocation support
- explainable scoring
- strategy-aware ranking across income, resale, and hybrid modes
- solver benchmarking across greedy, annealing, genetic, and experimental QUBO-like paths

## Current capabilities
- canonical property schema
- CSV property loading
- hard-constraint filtering
- weighted scoring model
- explanation summaries for ranked properties
- greedy baseline ranking
- early annealing path
- reproducible synthetic dataset generation
- CLI demo ranking flows

## Workspace
- `estate-opt-core`: canonical property schema, constraints, CSV loading
- `estate-opt-scoring`: scoring model, strategy-aware explainability primitives
- `estate-opt-solvers`: greedy baseline, annealing prototype, and synthetic fixture generation
- `estate-opt-cli`: command-line interface for ranking workflows

## Example commands
```bash
cargo run -p estate-opt-cli -- demo-rank --city los-angeles --budget 1500000 --count 1000 --strategy hybrid
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --city los-angeles --budget 1500000 --top 3 --strategy income
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --format markdown --strategy resale
```

More hands-on examples live in `examples/USAGE.md`.

## Rendered examples

### Text output
```text
la-001 | silver-lake | Income | score 0.565 | la-001 in silver-lake scored 0.565 in income mode with yield 0.068, liquidity 0.567, appreciation 0.666, resale spread 0.112, risk penalty 0.056, and repair penalty 0.040.
```

### JSON output
```json
[
  {
    "property": {
      "id": "la-002",
      "city": "los-angeles",
      "locality": "santa-monica",
      "price": 1650000.0,
      "expected_rent": 96000.0,
      "cap_rate": 0.07,
      "expected_sale_price": 1910000.0,
      "appreciation_score": 0.78,
      "vacancy_risk": 0.04,
      "repair_cost": 0.06,
      "liquidity_score": 0.88,
      "holding_horizon_months": 72
    },
    "breakdown": {
      "total": 1.4585454545454545,
      "strategy_mode": "resale",
      "yield_component": 0.07,
      "liquidity_component": 0.616,
      "appreciation_component": 0.7020000000000001,
      "resale_spread_component": 0.09454545454545456,
      "risk_penalty": 0.032,
      "repair_penalty": 0.024
    },
    "explanation": {
      "summary": "la-002 in santa-monica scored 1.459 in resale mode with yield 0.070, liquidity 0.616, appreciation 0.702, resale spread 0.095, risk penalty 0.032, and repair penalty 0.024.",
      "strengths": [
        "strong appreciation outlook for resale scenarios",
        "above-average liquidity profile"
      ],
      "tradeoffs": []
    }
  }
]
```

### Markdown report output
```markdown
| id | locality | strategy | score | summary |
| --- | --- | --- | ---: | --- |
| la-002 | santa-monica | Resale | 1.459 | la-002 in santa-monica scored 1.459 in resale mode with yield 0.070, liquidity 0.616, appreciation 0.702, resale spread 0.095, risk penalty 0.032, and repair penalty 0.024. |
| ny-002 | park-slope | Resale | 1.454 | ny-002 in park-slope scored 1.454 in resale mode with yield 0.066, liquidity 0.623, appreciation 0.729, resale spread 0.118, risk penalty 0.024, and repair penalty 0.028. |
```

See `examples/USAGE.md` for more command variants and explanations.

## Principles
- practical first, not hype first
- classical baselines before experimental heuristics
- explainability is a feature, not an afterthought
- domain assumptions must stay explicit and reviewable
- Lean alignment, if added later, should mirror specs and principles rather than claim full floating-point verification

## Near-term roadmap
1. normalization refinements and multi-objective controls
2. stronger annealing and genetic search solvers
3. benchmark harnesses with manual acceptance criteria
4. README variants for open-source, investor, and proptech audiences
5. strategy-specific demos for income, resale, and hybrid workflows

## Technical notes
- See `docs/scoring-model.md` for a lightweight formula-level explanation of the current scoring approach.

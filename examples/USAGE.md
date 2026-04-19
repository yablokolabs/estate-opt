# Usage Examples

## Rank from a CSV file
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --city los-angeles --budget 5000000 --top 3
```

### Example text output
```text
ny-002 | park-slope | score 0.637 | ny-002 in park-slope scored 0.637 with yield 0.066, liquidity 0.623, risk penalty 0.024, and repair penalty 0.028.
la-002 | santa-monica | score 0.630 | la-002 in santa-monica scored 0.630 with yield 0.070, liquidity 0.616, risk penalty 0.032, and repair penalty 0.024.
ny-001 | astoria | score 0.587 | ny-001 in astoria scored 0.587 with yield 0.071, liquidity 0.588, risk penalty 0.040, and repair penalty 0.032.
```

This shows the top-ranked matching properties, their score, and a short explanation of the score components.

## Rank a larger synthetic dataset
```bash
cargo run -p estate-opt-cli -- demo-rank --city los-angeles --budget 1500000 --count 1000 --seed 42
```

### What to expect
- repeated runs with the same seed should be reproducible
- you should see a handful of top-ranked synthetic properties
- this is useful for speed and regression testing before using real datasets

## Try annealing mode
```bash
cargo run -p estate-opt-cli -- demo-rank --city los-angeles --budget 1500000 --count 500 --solver annealing
```

### What to expect
- currently this is an early heuristic path
- output format matches greedy mode
- use it to compare ranking stability and future benchmark results

## JSON output for downstream tooling
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --format json --top 2
```

### Example JSON output
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

This is useful for APIs, notebooks, or later dashboard integrations.

## Markdown output for reports
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --format markdown --top 3
```

### Example Markdown output
```markdown
| id | locality | score | summary |
| --- | --- | ---: | --- |
| ny-002 | park-slope | 0.637 | ny-002 in park-slope scored 0.637 with yield 0.066, liquidity 0.623, risk penalty 0.024, and repair penalty 0.028. |
| la-002 | santa-monica | 0.630 | la-002 in santa-monica scored 0.630 with yield 0.070, liquidity 0.616, risk penalty 0.032, and repair penalty 0.024. |
| ny-001 | astoria | 0.587 | ny-001 in astoria scored 0.587 with yield 0.071, liquidity 0.588, risk penalty 0.040, and repair penalty 0.032. |
```

This format is useful for README snippets, investor notes, and lightweight reporting.

## Explore a different city
```bash
cargo run -p estate-opt-cli -- demo-rank --city new-york --budget 1800000 --count 250 --seed 7
```

## What the output means
Each result includes:
- property id
- locality
- total score
- explanation summary describing why it ranked there

## Expected CSV columns
```text
id,city,locality,price,expected_rent,cap_rate,vacancy_risk,repair_cost,liquidity_score,holding_horizon_months
```

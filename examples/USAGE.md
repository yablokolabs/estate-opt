# Usage Examples

## Rank from a CSV file
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --city bengaluru --budget 5000000 --top 3
```

### Example text output
```text
blr-002 | whitefield | score 0.540 | blr-002 in whitefield scored 0.540 with yield 0.071, liquidity 0.553, risk penalty 0.048, and repair penalty 0.036.
blr-001 | indiranagar | score 0.536 | blr-001 in indiranagar scored 0.536 with yield 0.074, liquidity 0.574, risk penalty 0.064, and repair penalty 0.048.
hyd-001 | gachibowli | score 0.515 | hyd-001 in gachibowli scored 0.515 with yield 0.076, liquidity 0.539, risk penalty 0.056, and repair penalty 0.044.
```

This shows the top-ranked matching properties, their score, and a short explanation of the score components.

## Rank a larger synthetic dataset
```bash
cargo run -p estate-opt-cli -- demo-rank --city bengaluru --budget 5000000 --count 1000 --seed 42
```

### What to expect
- repeated runs with the same seed should be reproducible
- you should see a handful of top-ranked synthetic properties
- this is useful for speed and regression testing before using real datasets

## Try annealing mode
```bash
cargo run -p estate-opt-cli -- demo-rank --city bengaluru --budget 5000000 --count 500 --solver annealing
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

This is useful for APIs, notebooks, or later dashboard integrations.

## Markdown output for reports
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --format markdown --top 3
```

### Example Markdown output
```markdown
| id | locality | score | summary |
| --- | --- | ---: | --- |
| blr-002 | whitefield | 0.540 | blr-002 in whitefield scored 0.540 with yield 0.071, liquidity 0.553, risk penalty 0.048, and repair penalty 0.036. |
| blr-001 | indiranagar | 0.536 | blr-001 in indiranagar scored 0.536 with yield 0.074, liquidity 0.574, risk penalty 0.064, and repair penalty 0.048. |
| hyd-001 | gachibowli | 0.515 | hyd-001 in gachibowli scored 0.515 with yield 0.076, liquidity 0.539, risk penalty 0.056, and repair penalty 0.044. |
```

This format is useful for README snippets, investor notes, and lightweight reporting.

## Explore a different city
```bash
cargo run -p estate-opt-cli -- demo-rank --city hyderabad --budget 4500000 --count 250 --seed 7
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

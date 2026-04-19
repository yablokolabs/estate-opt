# Usage Examples

## Income-focused ranking from a CSV file
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --city los-angeles --budget 1500000 --top 3 --strategy income
```

## Resale-focused ranking
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --strategy resale --format markdown
```

## Hybrid ranking
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --strategy hybrid --format json
```

## Larger synthetic dataset
```bash
cargo run -p estate-opt-cli -- demo-rank --city los-angeles --budget 1500000 --count 1000 --seed 42 --strategy hybrid
```

## Annealing mode
```bash
cargo run -p estate-opt-cli -- demo-rank --city new-york --budget 1800000 --count 500 --solver annealing --strategy resale
```

## Strategy meanings
- `income`: prioritizes yield, liquidity, and downside controls for rent-oriented evaluation
- `resale`: prioritizes appreciation and sale spread for sell-side or flip-oriented evaluation
- `hybrid`: balances both rental and resale signals

## Example income text output
```text
ny-002 | park-slope | Income | score 0.637 | ny-002 in park-slope scored 0.637 in income mode with yield 0.066, liquidity 0.623, appreciation 0.729, resale spread 0.118, risk penalty 0.024, and repair penalty 0.028.
```

## Example resale Markdown output
```markdown
| id | locality | strategy | score | summary |
| --- | --- | --- | ---: | --- |
| ny-002 | park-slope | Resale | 1.418 | ny-002 in park-slope scored 1.418 in resale mode with yield 0.066, liquidity 0.623, appreciation 0.729, resale spread 0.118, risk penalty 0.024, and repair penalty 0.028. |
| la-002 | santa-monica | Resale | 1.357 | la-002 in santa-monica scored 1.357 in resale mode with yield 0.070, liquidity 0.616, appreciation 0.702, resale spread 0.095, risk penalty 0.032, and repair penalty 0.024. |
```

## Example resale JSON output
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
      "expected_sale_price": 2130000.0,
      "appreciation_score": 0.81,
      "vacancy_risk": 0.03,
      "repair_cost": 0.07,
      "liquidity_score": 0.89,
      "holding_horizon_months": 84
    },
    "breakdown": {
      "total": 1.4179775280898876,
      "strategy_mode": "resale",
      "yield_component": 0.066,
      "liquidity_component": 0.623,
      "appreciation_component": 0.7290000000000001,
      "resale_spread_component": 0.11797752808988764,
      "risk_penalty": 0.024,
      "repair_penalty": 0.028000000000000004
    },
    "explanation": {
      "summary": "ny-002 in park-slope scored 1.418 in resale mode with yield 0.066, liquidity 0.623, appreciation 0.729, resale spread 0.118, risk penalty 0.024, and repair penalty 0.028.",
      "strengths": [
        "strong appreciation outlook for resale scenarios",
        "above-average liquidity profile"
      ],
      "tradeoffs": []
    }
  }
]
```

## Expected CSV columns
```text
id,city,locality,price,expected_rent,cap_rate,expected_sale_price,appreciation_score,vacancy_risk,repair_cost,liquidity_score,holding_horizon_months
```

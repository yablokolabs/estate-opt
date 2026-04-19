# Usage Examples

## Rank from a CSV file
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --city bengaluru --budget 5000000 --top 3
```

## Rank a larger synthetic dataset
```bash
cargo run -p estate-opt-cli -- demo-rank --city bengaluru --budget 5000000 --count 1000 --seed 42
```

## Try annealing mode
```bash
cargo run -p estate-opt-cli -- demo-rank --city bengaluru --budget 5000000 --count 500 --solver annealing
```

## JSON output for downstream tooling
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --format json
```

## Markdown output for reports
```bash
cargo run -p estate-opt-cli -- rank examples/sample_properties.csv --format markdown
```

## Explore a different city
```bash
cargo run -p estate-opt-cli -- demo-rank --city hyderabad --budget 4500000 --count 250 --seed 7
```

## What the output means
Each line shows:
- property id
- locality
- total score
- explanation summary describing why it ranked where it did

## Sample text output
```text
prop-00019 | core | score 0.573 | prop-00019 in core scored 0.573 with yield 0.086, liquidity 0.569, risk penalty 0.056, and repair penalty 0.026.
```

## Expected CSV columns
```text
id,city,locality,price,expected_rent,cap_rate,vacancy_risk,repair_cost,liquidity_score,holding_horizon_months
```

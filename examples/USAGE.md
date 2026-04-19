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

## Expected CSV columns
```text
id,city,locality,price,expected_rent,cap_rate,expected_sale_price,appreciation_score,vacancy_risk,repair_cost,liquidity_score,holding_horizon_months
```

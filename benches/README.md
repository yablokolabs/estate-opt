# Benchmark plan

Planned benchmark families:
- synthetic property ranking at 100, 1_000, and 10_000 rows
- greedy baseline runtime and score distribution
- annealing runtime and stability against the same seeds
- later comparisons against genetic search

## Example benchmark commands
```bash
cargo run -p estate-opt-cli -- demo-rank --count 100 --solver greedy
cargo run -p estate-opt-cli -- demo-rank --count 1000 --solver annealing --format json
cargo run -p estate-opt-cli -- demo-rank --count 10000 --solver greedy --format markdown
```

Acceptance criteria remain manual and should not be delegated to scaffolding bots.

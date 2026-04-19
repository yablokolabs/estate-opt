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

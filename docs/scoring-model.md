# Scoring Model

This document explains the current scoring model in a lightweight technical way.

## Goal
The scoring layer gives each property a comparable rank based on return-oriented and risk-aware factors, while keeping hard constraints separate.

Hard constraints answer:
- is this property allowed at all?

Scoring answers:
- among the allowed properties, which ones look stronger and why?

## Current score shape
The current prototype uses a weighted additive score:

```text
score = yield_component + liquidity_component - risk_penalty - repair_penalty
```

Where the current implementation maps to:

```text
yield_component = cap_rate * yield_weight
liquidity_component = liquidity_score * liquidity_weight
risk_penalty = vacancy_risk * risk_weight
repair_penalty = repair_cost * repair_cost_weight
```

And therefore:

```text
total_score = (cap_rate * yield_weight)
            + (liquidity_score * liquidity_weight)
            - (vacancy_risk * risk_weight)
            - (repair_cost * repair_cost_weight)
```

## Hard constraints vs soft preferences
Examples of hard constraints:
- max budget
- required city
- minimum liquidity score
- maximum vacancy risk

These are applied before ranking.

The weighted score is then used only on properties that survive the hard filter.

## Why this is useful
This structure keeps the system explainable:
- agents can see why a property was excluded
- users can see why a surviving property ranked higher
- future solvers can optimize against a clear objective

## Example intuition
A property can rank well if it has:
- strong cap rate
- good liquidity
- acceptable vacancy risk
- manageable repair burden

A property can rank lower if:
- repair cost is high
- liquidity is weak
- vacancy risk drags down the total score

## Important limitation
This is intentionally a simple starting model.

It is **not yet**:
- a fully calibrated market model
- a region-specific underwriting engine
- a learned model from real transaction history
- a guarantee of globally optimal investing decisions

## Why no heavy math in the README
The README is meant to stay accessible for open-source users, proptech operators, brokers, and investors.
This document is the better place for formulas and optimization notes.

## Future extensions
Later versions may add:
- normalized feature scaling
- multi-objective optimization
- locality diversification penalties
- time-horizon-aware objectives
- portfolio-level constraints
- experimental QUBO-style binary formulations

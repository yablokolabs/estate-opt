# Scoring Model

This document explains the current scoring model in a lightweight technical way.

## Goal
The scoring layer gives each property a comparable rank based on return-oriented and risk-aware factors, while keeping hard constraints separate.

Hard constraints answer:
- is this property allowed at all?

Scoring answers:
- among the allowed properties, which ones look stronger and why?

## Current score shape
The current prototype uses a weighted additive score.

### Rendered form
$$
S = Y + L - V - R
$$

Where:

$$
Y = w_y \cdot c
$$

$$
L = w_l \cdot \ell
$$

$$
V = w_v \cdot v
$$

$$
R = w_r \cdot r
$$

So the full score is:

$$
S = (w_y \cdot c) + (w_l \cdot \ell) - (w_v \cdot v) - (w_r \cdot r)
$$

## Variable meanings
- $S$: total score
- $c$: cap rate
- $\ell$: liquidity score
- $v$: vacancy risk
- $r$: repair cost
- $w_y$: yield weight
- $w_l$: liquidity weight
- $w_v$: vacancy-risk weight
- $w_r$: repair-cost weight

## Hard constraints vs soft preferences
Examples of hard constraints:
- maximum budget
- required city
- minimum liquidity score
- maximum vacancy risk

These are applied before ranking.

Formally, if a property $p$ does not satisfy the hard constraint set $\mathcal{C}$, then it is excluded before scoring:

$$
p \notin \mathcal{C} \implies p \text{ is filtered out}
$$

The weighted score is only used on properties that survive the hard filter.

## Why this is useful
This structure keeps the system explainable:
- agents can see why a property was excluded
- users can see why a surviving property ranked higher
- future solvers can optimize against a clear objective

## Example intuition
A property tends to rank well when:
- cap rate is strong
- liquidity score is strong
- vacancy risk is lower
- repair burden is lower

A property tends to rank worse when:
- repair cost is high
- liquidity is weak
- vacancy risk materially drags down the total

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

A more structured portfolio objective could eventually look like:

$$
\max_{x \in \{0,1\}^n} \sum_{i=1}^{n} x_i S_i - \lambda D(x)
$$

Where:
- $x_i$ indicates whether property $i$ is selected
- $S_i$ is the property-level score
- $D(x)$ is a diversification or concentration penalty
- $\lambda$ controls the trade-off between raw score and diversification

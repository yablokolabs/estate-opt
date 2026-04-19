# Scoring Model

This document explains the current scoring model in a lightweight technical way.

## Goal
The scoring layer gives each property a comparable rank based on return-oriented and risk-aware factors, while keeping hard constraints separate.

Hard constraints answer:
- is this property allowed at all?

Scoring answers:
- among the allowed properties, which ones look stronger and why for a chosen strategy?

## Strategy modes
The prototype currently supports three modes:
- **Income**: prioritize rental yield, liquidity, and downside control
- **Resale**: prioritize appreciation potential and sale spread
- **Hybrid**: balance both income and resale signals

## Current score shape
The current prototype uses a weighted additive score.

### Income mode
$$
S_{\mathrm{income}} = Y + L - V - R
$$

### Resale mode
$$
S_{\mathrm{resale}} = A + P + L - V - R
$$

### Hybrid mode
$$
S_{\mathrm{hybrid}} = Y + A + P + L - V - R
$$

Where:

$$
Y = w_y \cdot c
$$

$$
L = w_l \cdot \ell
$$

$$
A = w_a \cdot a
$$

$$
P = w_p \cdot \frac{s - p}{p}
$$

$$
V = w_v \cdot v
$$

$$
R = w_r \cdot r
$$

## Variable meanings
- $c$: cap rate
- $\ell$: liquidity score
- $a$: appreciation score
- $s$: expected sale price
- $p$: current purchase price
- $v$: vacancy risk
- $r$: repair cost
- $w_y$: yield weight
- $w_l$: liquidity weight
- $w_a$: appreciation weight
- $w_p$: resale-spread weight
- $w_v$: vacancy-risk weight
- $w_r$: repair-cost weight

## Hard constraints vs soft preferences
Examples of hard constraints:
- maximum budget
- required city
- minimum liquidity score
- maximum vacancy risk

These are applied before ranking.

Formally, if a property $x$ does not satisfy the hard constraint set $\mathcal{C}$, then it is excluded before scoring:

$$
x \notin \mathcal{C} \implies x \text{ is filtered out}
$$

The weighted score is only used on properties that survive the hard filter.

## Why this is useful
This structure keeps the system explainable:
- agents can see why a property was excluded
- users can see why a surviving property ranked higher
- future solvers can optimize against a clear objective
- the same property can be evaluated differently depending on the user strategy

## Important limitation
This is intentionally a simple starting model.

It is **not yet**:
- a fully calibrated market model
- a region-specific underwriting engine
- a learned model from real transaction history
- a guarantee of globally optimal investing decisions

## Future extensions
Later versions may add:
- normalized feature scaling
- multi-objective optimization
- locality diversification penalties
- time-horizon-aware objectives
- portfolio-level constraints
- experimental QUBO-style binary formulations

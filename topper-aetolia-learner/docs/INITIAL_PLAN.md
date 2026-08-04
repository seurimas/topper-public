# Topper Aetolia Learner - Initial Plan

## Vision

`topper-aetolia-learner` will be a local-first experimentation package for optimizing combat plans using TypeScript ML utilities and high-speed Rust simulation primitives compiled to WebAssembly.

The learner should make it easy to:

1. Define a simplified combat scenario.
2. Generate and score candidate plans.
3. Compare outcomes quickly using existing `Timeline` and observable/event semantics.
4. Iterate on simulation fidelity without blocking frontend experimentation.

## Objectives

1. Build a clean Svelte + WASM dev loop for rapid iteration.
2. Reuse and extend existing `topper-aetolia` timeline state transitions.
3. Expand `ActiveTransition::simulate` coverage across actions to reduce gaps and `todo!()` fallbacks.
4. Add TS-side optimization loops (greedy, beam search, Monte Carlo, and eventually model-guided ranking).
5. Keep this package private and excluded from public mirroring.

## Proposed Architecture

### Frontend (Svelte + TypeScript)

- Scenario editor: attacker/target state, horizon, action candidates, constraints.
- Simulation runner: calls WASM exports and receives deterministic result payloads.
- Experiment dashboard:
  - per-plan score tables
  - confidence/branch-weight details
  - failure traces and unresolved transition flags
- ML layer (TS): utility modules for search/ranking and feature extraction.

### Backend (Rust -> WASM)

- Thin, stable WASM API surface:
  - `simulate_plan(scenario_json) -> result_json`
  - future: `simulate_batch`, `extract_features`, `explain_decision`
- Core responsibilities:
  - scenario normalization
  - timeline bootstrapping
  - action rollout
  - probabilistic branch accounting
- Shared engine intent:
  - maximize reuse of `topper-aetolia` timeline/observables modules
  - avoid duplicating combat rules in TypeScript

## Phase Plan

## Phase 0 - Scaffold (current)

1. Create package with Svelte + TypeScript + Vite.
2. Create custom Rust WASM crate under this package.
3. Wire frontend to call WASM and display results.
4. Add private-path exclusion in mirror workflow.

## Phase 1 - Simulation Baseline

1. Define canonical scenario schema:
  - actor snapshots
  - balances/cooldowns
  - selected afflictions/defenses
  - action candidate list
2. Add deterministic baseline scorer and sanity fixtures.
3. Build fixtures for short scripted combat snippets.
4. Validate timeline progression against expected observations.

## Phase 2 - ActiveTransition Expansion

1. Audit all actions with missing or shallow `simulate` implementations.
2. Prioritize high-impact classes/rotations first.
3. Introduce reusable simulation helpers for common action patterns.
4. Add tests to enforce no regression in simulated observation output.

## Phase 3 - Optimization and Learning Utilities

1. Add TS search strategies:
  - greedy baseline
  - beam search
  - stochastic rollouts
2. Add feature extraction from simulation traces.
3. Introduce objective components:
  - kill pressure
  - lock potential
  - survivability
  - balance efficiency
4. Compare strategy performance over fixture suites.

## Phase 4 - Evaluation and Tooling

1. Build repeatable benchmark command(s).
2. Add result persistence for scenario comparison.
3. Add visualization for branch trees and score deltas.
4. Document experiment protocol and acceptance gates.

## Data and Testing Strategy

1. Start with deterministic fixture snapshots for quick regression checks.
2. Add property checks for basic invariants (time monotonicity, balance bounds).
3. Keep unit tests close to simulation boundaries.
4. Add integration tests for end-to-end scenario to score output.

## Key Risks

1. Incomplete `simulate` coverage can bias optimizer outcomes.
2. Divergence between runtime combat logic and simulation model.
3. Overfitting optimization to a narrow fixture set.
4. WASM serialization overhead for very large batch runs.

## Immediate Next Tasks

1. Formalize v1 scenario/result schemas and version them.
2. Add `simulate_batch` export for evaluating many plans per call.
3. Add first fixture pack (small, medium, adverse states).
4. Start ActiveTransition coverage audit checklist by class/action.

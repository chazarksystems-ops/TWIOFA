# Gemini Optimization Review

## Status
- **Normative status:** Optimization-priority guidance only.
- **Scope:** Preserve useful Gemini ideas while correcting priority and scope.

## What this document does not authorize
- Does not authorize Optimization Pass 0 implementation.
- Does not authorize Slice 3.
- Does not authorize MCMC, MCTS, GPU, Margolus/block CA, active chunks, or gameplay optimization work now.

## Verdict
Gemini is directionally useful, but too early-heavy on Monte Carlo/MCMC/MCTS/GPU. TWIOFA should first build deterministic measurement and verification.

## Promote now
- Deterministic headless harness.
- Replay corpus.
- Invariant checks.
- Fixture stress tests.
- Micro-counters.
- Active-region instrumentation.
- Baseline comparison.

## Promote later
- Active chunk/sleep lists after metrics show wasted scanning.
- Staged deltas for collapse/moisture/scent.
- Bounded parameter sweeps.
- Adversarial command-sequence search.

## Defer
- MCMC balancing until tunable parameters and objective metrics exist.
- MCTS until command API and interaction space are richer.
- GPU compute until CPU correctness and profiling justify it.
- Margolus/block CA until rule shape demands parallel CA partitioning.

## Reject for Chunk 0
- “GPU is necessary.”
- “MCMC before deterministic harness.”
- “MCTS before command semantics are stable.”
- Broad Noita imitation as architecture.
- Any optimization that weakens receipts, determinism, or readability.

## Optimization order
1. Measure.
2. Verify.
3. Stress.
4. Compare.
5. Optimize.

## How Gemini ideas map to TWIOFA
| Gemini idea | TWIOFA handling |
|---|---|
| Active chunks | Future candidate after active-region metrics. |
| Sleep lists | Future candidate after stable/inert cell tracking. |
| Monte Carlo | Future parameter exploration. |
| MCTS | Future adversarial command search. |
| Margolus | Future GPU/parallel CA research only. |
| Headless runs | Immediate harness feature. |

## Required note
Optimization Pass 0 is not gameplay optimization. It is instrumentation and proof infrastructure.

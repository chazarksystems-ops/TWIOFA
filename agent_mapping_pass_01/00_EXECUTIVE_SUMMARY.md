# 00 — Executive Summary: TWIOFA Chunk 0 Spec Intelligence Map

## Status

**v0.3.2 locked-contract candidate. No implementation authorized.**

---

## What This Is

TWIOFA Chunk 0 is the foundational material-substrate prototype for *The World Is on FIRE Ants*, a game exploring colony-scale ant strategy inside a Noita-inspired physically simulated yard. Chunk 0 specifies a single 128×128 cell grid—a bounded, deterministic sandbox—where soil, tunnels, water, moisture, scent trails, carcass material, Sourback residue, and structural collapse all interact through simple, testable rules. A single ant group (representing the colony's effort front) issues commands—dig, scout, forage, return, avoid—that physically alter cells on the grid. The simulation is Rust-owned, rendered via a canvas/grid visualizer (Tauri + HTML Canvas default, minifb/pixels fallback), and guarded by a strict epistemic boundary: WorldTruth (what physically exists) is separated from ColonyPerception (what the colony has observed and interpreted), with hidden truths withheld from the default player-facing view. A perception event ledger records observations, misreads, and reframes. Every command produces a deterministic CommandReceipt with chunk hashes for replay verification. The spec explicitly gates all implementation behind Chaz's approval of 12 open decisions (D1–D12) and a build-readiness checklist.

## What This Is Not

This is not the full TWIOFA game. It is not a card dashboard, not a menu-driven zone manager, not a procedural world generator, not a Noita clone, not a GPU compute project, not an Unreal integration, and not a full ant AI simulation. It is one chunk proving that a material substrate can physically change under colony pressure while maintaining deterministic replay and epistemic integrity. If Chunk 0 succeeds, later chunks add dirty-region tracking, richer pheromone fields, Field Notes UI, and eventually Unreal-quality presentation. If Chunk 0 fails—by reverting to cards, leaking hidden truth, or losing determinism—later chunks cannot be trusted.

---

## Core Direction Summary

| Axis | Chunk 0 Position |
|---|---|
| Substrate | Noita-inspired material-yard, not abstract zones |
| Scale | One 128×128 chunk, 16,384 cells |
| Simulation ownership | Rust-owned, CPU-only, single-threaded tick |
| Visualizer | Canvas/grid central surface; debug panels support it |
| Epistemic model | WorldTruth → RenderFrame → ColonyView; DevTruth toggle for debug |
| Ant model | Single-coordinate ant group, greedy movement, no A* |
| Perception | Perception Event Ledger; full Field Notes deferred |
| Determinism | Canonical serialization hash, stable tick order, no wall-clock |
| Implementation gate | D1–D12 + build-readiness gate, all requiring Chaz approval |
| Anti-pattern guard | Card-dashboard, GPU compute, procedural gen, Unreal—all banned |

---

## Spec Provenance

| Version | Origin | Role |
|---|---|---|
| v0.3.1 | Gemini DOCX export | Initial structured spec with some gaps |
| v0.3.2 | Correction pass over v0.3.1 | Fixed terminology drift, filled traceability/risk, hardened boundaries |
| v0.3.2 corrected workspace | Current workspace | Canonical spec candidate awaiting Chaz approval |

---

## Implementation Authorization

```
Status: NOT AUTHORIZED
Reason: D1–D12 require Chaz approval; build-readiness gate has not passed.
```

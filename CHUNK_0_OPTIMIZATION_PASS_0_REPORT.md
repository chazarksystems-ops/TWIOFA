# Chunk 0 — Optimization Pass 0 Report

## Overview

Optimization Pass 0 adds the `chunk0_harness` binary and the `harness` library module,
providing a deterministic test corpus and stress-local-fixtures runner. No simulation
logic was changed during this pass.

---

## Files Added

| File | Purpose |
|---|---|
| `src/harness.rs` | Invariant checker, corpus runner, stress runner |
| `src/bin/chunk0_harness.rs` | CLI binary exposing `run-corpus` and `stress-local-fixtures` subcommands |
| `src/lib.rs` | Added `pub mod harness` export |

---

## Invariant Checker (`check_invariants`)

Verifies per-run that:
1. `sim.chunk.cells.len() == 16384`
2. Boundary ring (x=0, x=127, y=0, y=127) is Stone with support=255
3. Default RenderFrame (dev_mode=false, sourback_earned=false) does not expose `SourbackBitter` text in `known_perception_marker` (only `"bitter/yellow residue"` or `"Sourback-associated"`)
4. `dev_residue` is `None` in colony mode

---

## Corpus Scripts (`run_corpus`) — 12 Scripts

| Script | Setup | Commands | Purpose |
|---|---|---|---|
| `RESET_ONLY` | none | none | stable initial hash |
| `BASIC_DIG` | ant at (45,100) | DigTunnel(44,100) | valid adjacent dig |
| `BOUNDARY_DIG_BLOCKED` | ant at (1,5) | DigTunnel(0,5) | boundary block |
| `MOVE_CARDINAL` | ant at (5,5) | Scout(10,5) + Step(3) | cardinal movement |
| `SOURBACK_ENTRY` | ant at (84,27) workers=10 conf=200 | Forage(84,33) | transition entry loss |
| `SOURBACK_SLOWDOWN` | ant at (85,30) | Scout(95,30) | odd-tick slowdown |
| `SOURBACK_REENTRY` | custom residue fixture | Step(6) | re-entry once per command |
| `BLOCKED_MOVEMENT` | sealed pocket at (10,10) | Step(3) | Blocked once per command |
| `GREEDY_TIE` | ant at (5,5) | Scout(7,7) | x-first tie-break |
| `LOW_WORKERS` | ant at (84,27) workers=2 conf=200 | Forage(84,33) | workers → 0 |
| `LOW_CONFIDENCE` | ant at (84,27) workers=10 conf=10 | Forage(84,33) | confidence saturates to 0 |
| `RENDERFRAME_BOUNDARY` | none | none | no hidden truth leak |

---

## Stress-Local-Fixtures (`stress_local_fixtures`)

Deterministic LCG: `state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)`

4 fixture variants (selected by `lcg_next % 4`):
- 0: Open-path scout
- 1: Residue-entry forage
- 2: Blocked pocket
- 3: Boundary probe

Invariant check after each case. Failure budget: first 5 failures logged; full count reported.

---

## Validation Results

```text
cargo run --bin chunk0_harness -- run-corpus
  [PASS] RESET_ONLY        hash_after=459e899bca89
  [PASS] BASIC_DIG         hash_after=70f223a5a08e
  [PASS] BOUNDARY_DIG_BLOCKED  hash_after=e69df0097d56
  [PASS] MOVE_CARDINAL     hash_after=62773f6d75fc
  [PASS] SOURBACK_ENTRY    workers=7 conf=184 pos=(84,28)
  [PASS] SOURBACK_SLOWDOWN pos=(85,30)
  [PASS] SOURBACK_REENTRY  workers=97 conf=239 pos=(10,15)
  [PASS] BLOCKED_MOVEMENT  pos=(10,10)
  [PASS] GREEDY_TIE        pos=(6,5)
  [PASS] LOW_WORKERS       workers=0
  [PASS] LOW_CONFIDENCE    conf=0
  [PASS] RENDERFRAME_BOUNDARY
  Corpus result: ALL PASSED (12/12)

cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
  stress-local-fixtures seed=42 cases=100: ALL PASSED (100/100)
```

---

## Output Artifacts

Results written to `optimization_runs/latest/`:
- `run_corpus_summary.txt`
- `run_corpus_hashes.csv`
- `stress_local_fixtures_summary.txt`

---

## No Simulation Changes

No simulation rules, canonical hash, or game logic were altered during Optimization Pass 0.
This pass adds observability only.

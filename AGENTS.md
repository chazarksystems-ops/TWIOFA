# AGENTS.md — TWIOFA Repository

## Repository Purpose

TWIOFA Chunk 0 verified playable/testable deterministic slice.

This is a deterministic 128×128 yard-substrate simulation in Rust. It proves movement, digging, Sourback residue risk, collapse, water/moisture, scent, harvest, return-home loop, deterministic receipts, harness validation, and CLI smoke play.

## Normative Source Hierarchy

1. `README.md` and `AGENTS.md` — repo workflow and constraints
2. `CHUNK_0_PLAYABLE_SLICE_REPORT.md` — acceptance truth
3. `CHUNK_0_IMPLEMENTATION_REPORT.md` — full implementation history
4. `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md` — harness documentation
5. `spec_pack/` — corrected v0.3.3 Chunk 0 spec
6. `docs/elevated_reasoning/` — elevated reasoning and Slice 2 freeze
7. `docs/playable_slice_completion/` — bounded completion path
8. `agent_mapping_pass_01/` — reconciled mapping / audit support
9. `chunk0_rust_scaffold/` — tests and code as implementation truth

## Non-Normative Sources

- `archive/source_review/` — historical provenance only; do not implement from this folder
- `source_review/` — same as above if present outside archive
- Old patch folders (`twiofa_claude_code_docs_patch/`, `twiofa_playable_slice_completion_docs_patch_refined/`)
- Pasted logs or older brainstorms unless explicitly promoted

## Rules for All Agents

### Scope
- Do not implement from `source_review/` or `archive/source_review/`.
- Do not reintroduce card-dashboard gameplay.
- Do not broaden Chunk 0 into full-game canon.
- Do not refactor simulation logic unless explicitly requested by Chaz.
- Do not treat Chunk 0 as the final, permanent game design.

### Simulation Integrity
- Preserve deterministic simulation at all times.
- No random or nondeterministic behavior in authoritative simulation paths.
- No wall time (`std::time`) in simulation logic — use explicit `tick_index` counters.
- No floating point (`f32`/`f64`) in authoritative state — all state uses `u8`, `u32`, `u64`.
- No `HashMap` iteration in deterministic simulation paths — use `Vec` or array iteration only.

### Code Changes
- Any Rust behavior change requires:
  1. Tests covering the changed behavior
  2. An update to `CHUNK_0_IMPLEMENTATION_REPORT.md` or `reports/`
  3. Full validation pass (see below)
- Do not add new dependencies without justification in a report.
- Keep `Cargo.lock` committed — do not add it to `.gitignore`.

### Deferred (not in Chunk 0 scope)
- Do not add GPU compute, Margolus neighborhood, block CA.
- Do not add MCTS, MCMC, or Monte Carlo simulation.
- Do not add A* pathfinding.
- Do not add Tauri/polished UI.
- Do not add procedural full-yard generation.
- Do not add multi-ant group doctrine, economy, or combat.

All of the above are explicitly deferred until Chaz creates a future research or expansion branch.

### Repository Hygiene
- Do not commit `target/`, zip backups, `.claude/`, or patch folders.
- Do not create or choose a license file without explicit Chaz approval.
- Do not force-push to `main`.
- Do not create version tags without explicit Chaz approval.

## Required Validation Before Claiming Success

Run from `chunk0_rust_scaffold/`:

```sh
cargo fmt --check
cargo check
cargo test
cargo run --bin chunk0_harness -- run-corpus
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
cargo run --bin chunk0_cli -- run-script smoke
```

Expected results:
- `cargo test`: 57 passed / 0 failed
- `run-corpus`: 12/12 passed
- `stress-local-fixtures`: 100/100 passed
- `run-script smoke`: `SMOKE_DETERMINISM: PASS`

Do not claim success if any of the above fails.

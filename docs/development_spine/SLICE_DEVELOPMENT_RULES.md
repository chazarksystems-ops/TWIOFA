# TWIOFA Slice Development Rules

## Purpose

TWIOFA development should be organized by categories, but implemented through playable slices.

A category owns a domain.

A slice proves that several domains work together in a testable player-facing loop.

## Core Rule

Do not build a whole category in isolation unless the task is explicitly a bounded audit, documentation pass, or tool-only support pass.

Preferred shape:

```text
Small playable loop
+ narrow file scope
+ explicit validation
+ Chaz-visible acceptance
```

## Slice Contract Required Fields

Every implementation slice must define:

```text
Slice ID
Slice title
Purpose
Player-visible outcome
Current evidence / repo status
Included scope
Excluded scope
Allowed files
Forbidden files
Required docs to read
Validation commands
Manual playtest checklist
Acceptance gates
Canon / drift risks
Stop conditions
Rollback / recovery note
```

## Slice IDs

Use stable IDs:

```text
S00_REPO_TRUTH_AND_SYNC_AUDIT
S01_PLAYABILITY_BASELINE
S02_CONTROLS_CAMERA_FEEL
S03_VISUAL_READABILITY
S04_SCOUT_FORAGE_ROUTE_READABILITY
S05_DIG_COLLAPSE_HOME_BLOCKING
S06_CARCASS_SOURBACK_OPENING
S07_SOFT_GATE_FAR_ROUTE
```

Additional slices may be added only when they have a clear player-visible outcome and do not duplicate an existing slice.

## Baseline Validation

When touching Rust implementation behavior, run from `chunk0_rust_scaffold/`:

```sh
cargo fmt --check
cargo check
cargo test
cargo run --bin chunk0_harness -- run-corpus
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
cargo run --bin chunk0_cli -- run-script smoke
```

Do not claim success if any required command fails.

## Docs-Only Slice Validation

For docs-only passes:

```text
- Confirm no implementation files were changed.
- Confirm docs state allowed/forbidden scope.
- Confirm no release-ready, public-demo, final-game, or canon-lock claims were introduced.
- Confirm all future work is still gated by Chaz approval.
```

## Status Language

Allowed cautious terms:

```text
verified scaffold
playable/testable deterministic slice
docs-only map
category spine
slice contract
candidate next pass
Chaz approval pending
implementation not yet started
```

Forbidden unless Chaz explicitly approves:

```text
final demo
release-ready
public demo
shippable
complete game
canonical full-game implementation
production architecture locked
```

## Category-to-Slice Rule

A slice should name which categories it touches.

Example:

```text
S05_DIG_COLLAPSE_HOME_BLOCKING
Touches:
- 03 World Map / Layers / Chunks
- 04 Materials / Terrain / Simulation
- 05 Colony Systems
- 07 Orders / AI / Resolution
- 08 UI / UX / Tactical Board
- 13 QA / Playtesting / Acceptance
```

This prevents the repo from being split into disconnected discipline silos.

## Forbidden Drift

Do not use a slice as permission to:

```text
- broaden Chunk 0 into full game canon;
- add full procedural yard generation;
- add GPU/WebGPU substrate proof inside the old scaffold without a new approved branch/contract;
- add networking, accounts, live service, or global maps;
- convert the game into a card dashboard;
- accept debug-grid visuals as the final visual target;
- bypass deterministic receipts and tests;
- move folders before a migration plan is approved.
```

## Current Known Audit Need

Before any graphical-playability implementation, run S00.

S00 must answer:

```text
Does `chunk0_game` exist in GitHub, another branch, a local-only state, or only in planning docs?
```

Until S00 answers that, graphical tasks must not assume a game binary exists in the active GitHub repo.

## Slice Acceptance Principle

A slice is accepted only when it answers:

```text
What can Chaz now do, see, or test that he could not reliably do before?
```

If the answer is only “the code compiles,” the task is not a playable slice. It may still be a validation pass, but it should not be labeled a playability milestone.

# TWIOFA Repo Category Map

## Purpose

This file defines the development categories TWIOFA should use before any physical repo split happens.

It does not move files.

It classifies areas of responsibility so future agents can work on small, reviewable, playable slices without drifting the project into a generic ant game, card dashboard, terminal-only tool, or debug-grid prototype.

## Category Rules

Each category must eventually have:

```text
1. Design intent
2. Current implementation status
3. Next build task
4. Acceptance test
5. Files allowed for that task
6. Files forbidden for that task
```

Categories are not milestones by themselves. A milestone should be a playable slice that crosses several categories.

---

# 01 — Design Spine / Game Identity

## Owns

```text
North star
Design philosophy
Open-yard structure
Packet / queenline / campaign frame
Board-first consequence
Physical truth / player interpretation
No linear acts
No moral labels
Canon-risk language
```

## Current repo locations

```text
README.md
AGENTS.md
spec_pack/
docs/elevated_reasoning/
docs/playable_slice_completion/
CHUNK_0_PLAYABLE_SLICE_REPORT.md
CHUNK_0_IMPLEMENTATION_REPORT.md
```

## Current status

Chunk 0 is a deterministic local scaffold and does not lock full-game canon.

## Acceptance question

```text
Does this change preserve TWIOFA's identity instead of becoming a generic ant simulation?
```

---

# 02 — Controls / Camera / Feel

## Owns

```text
Camera speed
Pan / zoom
Follow behavior
Surface / underground framing
Mouse targeting
Keyboard controls
Input repetition reduction
Guided controls
Playability feel
```

## Current repo locations

```text
Potential future graphical target under chunk0_rust_scaffold/src/bin/ if present
Potential future docs/playability/ if imported
```

## Current status

The GitHub repo currently verifies CLI and harness binaries. A graphical `chunk0_game` target must be audited before any control/camera task assumes it exists.

## Acceptance question

```text
Can Chaz comfortably read and shape the yard without fighting the camera or repetitive input?
```

---

# 03 — World Map / Layers / Chunks

## Owns

```text
Starting nest location
128x128 Chunk 0 yard substrate
Surface / underground relationship
Chunk boundaries
Routes
Soft gates
Landmarks
POI placement
Garden-bed or yard boundary assumptions when promoted
```

## Current repo locations

```text
chunk0_rust_scaffold/src/**
spec_pack/
CHUNK_0_IMPLEMENTATION_REPORT.md
```

## Acceptance question

```text
Does the world layout create decisions before the UI explains them?
```

---

# 04 — Materials / Terrain / Simulation

## Owns

```text
Soil
Loose soil
Tunnel
Collapse
Water
Moisture
Roots / stone / blocking material
Residue
Carcass material
Scent/material interactions
```

## Current repo locations

```text
chunk0_rust_scaffold/src/sim.rs
chunk0_rust_scaffold/src/chunk.rs
chunk0_rust_scaffold/tests/
CHUNK_0_PLAYABLE_SLICE_REPORT.md
```

## Current status

Chunk 0 validates movement, digging, Sourback residue risk, collapse, water/moisture, scent, harvest, return-home loop, deterministic receipts, harness validation, and CLI smoke play.

## Acceptance question

```text
When ants touch the world, does the world answer with deterministic physical consequence?
```

---

# 05 — Colony Systems

## Owns

```text
Queen / home concept
Worker group state
Brood placeholder if added later
Food carried
Food returned
Return-home loop
Worker loss
Alarm / colony status if added later
```

## Current repo locations

```text
chunk0_rust_scaffold/src/sim.rs
chunk0_rust_scaffold/src/events.rs
chunk0_rust_scaffold/src/perception.rs
chunk0_rust_scaffold/tests/
```

## Acceptance question

```text
Does the colony feel like a living body rather than a cursor?
```

---

# 06 — Creatures / Ecology / Hazards

## Owns

```text
Sourback residue
Future Sourback Caterpillar
Future visible hazards
Future neutral insects
Future predators
Future rival ants
Future weather/human-scale disturbances
```

## Current repo locations

```text
chunk0_rust_scaffold/src/** for existing Sourback residue behavior
spec_pack/
```

## Acceptance question

```text
Does the yard feel occupied by other systems and worlds, not just tiles?
```

---

# 07 — Orders / AI / Resolution

## Owns

```text
Scout
Forage
Dig
Return home
Guard if added
Avoid/Hold if added
Resolution receipts
Command result clarity
Board-first consequence hooks
```

## Current repo locations

```text
chunk0_rust_scaffold/src/sim.rs
chunk0_rust_scaffold/src/bin/chunk0_cli.rs
chunk0_rust_scaffold/src/harness.rs
chunk0_rust_scaffold/tests/
```

## Acceptance question

```text
Can the player issue colony-scale intent and see why it succeeded, failed, or changed the board?
```

---

# 08 — UI / UX / Tactical Board

## Owns

```text
Yard board
Nest panel
Order panel
Result card
Field Notes
Route display
Known/suspected/unknown state presentation
Why-action-failed messages
Accessibility presentation
```

## Current repo locations

```text
chunk0_rust_scaffold/src/perception.rs
chunk0_rust_scaffold/src/render.rs if present
future graphical target if present
```

## Current status

Current GitHub-visible implementation is CLI/harness oriented. Full tactical-board UI is not verified in GitHub.

## Acceptance question

```text
Does the interface help the player think without telling them what to think?
```

---

# 09 — Art / Visual Language

## Owns

```text
Ant-scale readability
Material readability
Terrain style
Food/home/ant hierarchy
Visual residue
No debug-grid acceptance
Atmosphere
Surface vs underground visual identity
```

## Current repo locations

```text
Potential future graphical target
Potential future docs/playability/ visual specs
```

## Acceptance question

```text
Can a player instantly understand what matters while still feeling depth and material consequence?
```

---

# 10 — Tools / Debug / Validation

## Owns

```text
Harness
Corpus
Stress fixtures
CLI / REPL
Smoke scripts
Render views
Receipts
Debug overlays
Future map/material inspectors
```

## Current repo locations

```text
chunk0_rust_scaffold/src/harness.rs
chunk0_rust_scaffold/src/bin/chunk0_harness.rs
chunk0_rust_scaffold/src/bin/chunk0_cli.rs
chunk0_rust_scaffold/tests/
CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md
```

## Acceptance question

```text
Can the game be tested, inspected, and reproduced without guessing?
```

---

# 11 — Save / Persistence / Campaign State

## Owns

```text
Packet autosave rules
Queenline state
Deep Hive instance state
Campaign memory
Field Notes persistence
Import/export
Deterministic seed identity
```

## Current repo locations

```text
Design docs only unless code is added later
```

## Acceptance question

```text
Does the game remember the right things without enabling outcome erasure?
```

---

# 12 — Performance / Architecture

## Owns

```text
Determinism
State representation
Update ordering
No nondeterministic simulation paths
Chunk scaling
Future GPU/WebGPU substrate proof boundaries
Future sparse/active-zone simulation
```

## Current repo locations

```text
AGENTS.md
chunk0_rust_scaffold/src/**
chunk0_rust_scaffold/tests/
TWIOFA-v0.01 reference docs when explicitly cited
```

## Acceptance question

```text
Can the game scale without throwing away the deterministic material-consequence spine?
```

---

# 13 — QA / Playtesting / Acceptance

## Owns

```text
Validation commands
Smoke tests
Manual playtest scripts
Acceptance scorecards
Regression checks
Chaz playtest gates
Status language
```

## Current repo locations

```text
README.md
AGENTS.md
CHUNK_0_PLAYABLE_SLICE_REPORT.md
CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md
chunk0_rust_scaffold/tests/
```

## Acceptance question

```text
Did this actually improve the playable game, and can the claim be validated?
```

---

# 14 — Playable Slices / Milestones

## Owns

```text
Cross-category playable increments
Slice contracts
Allowed/forbidden file scopes
Acceptance gates
Milestone reports
Chaz approval boundaries
```

## Current repo locations

```text
docs/development_spine/slices/
future slice contracts
```

## Acceptance question

```text
Does this slice combine categories into something testable by a human?
```

---

# Do Not Physically Move Yet

The following must not be moved in Pass 1:

```text
chunk0_rust_scaffold/
spec_pack/
docs/elevated_reasoning/
docs/playable_slice_completion/
agent_mapping_pass_01/
archive/
source_review/
root reports
```

A future physical split requires a separate approved migration plan, including validation commands and rollback instructions.

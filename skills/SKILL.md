---
name: twiofa_chunk0_spec_hardener
description: Review and harden TWIOFA Chunk 0 material-yard specs before implementation. Preserve Noita-like material substrate direction while preventing card-dashboard regression, premature Unreal/GPU scope, hidden truth leaks, and implementation before approval.
---

# TWIOFA Chunk 0 Spec Hardener

Use this skill when reviewing or expanding TWIOFA Chunk 0 documents.

## Core Goal

Chunk 0 must prove a visible, inspectable, deterministic 128x128 material-yard substrate.

The material grid is the main surface. Debug panels support the grid. Debug panels must never become the game.

## Non-Negotiable Gates

Reject or correct any doc/prompt that:

- authorizes implementation before Chaz approval;
- turns Chunk 0 into a card dashboard;
- hides the material grid behind panels/logs;
- adds full yard generation;
- adds Unreal integration;
- adds GPU-owned simulation or custom compute;
- adds A* pathfinding;
- adds multiplayer;
- leaks hidden WorldTruth into default ColonyView;
- treats food/carcass as a counter without physical cell change;
- removes deterministic hash/replay testing.

## Required Concepts

Every build-ready spec must include:

- D1-D12 decisions and approval status (see spec_pack/12 and agent_mapping_pass_01/03);
- exact chunk size and coordinate convention;
- exact cell schema;
- material semantics table;
- material interaction table;
- normative tick order (incl. double-buffer moisture);
- command contract;
- event / CommandReceipt schema (note: CommandReceipt = Rust/domain type; command_receipt = snake_case in serialized fields);
- RenderFrame / VisibleCell boundary;
- validation test matrix;
- traceability matrix;
- risk register;
- no-card UI compliance test;
- build-readiness gate (v0.3.3 cleanup improves readiness; implementation still requires explicit Chaz approval — no file authorizes "start building now").

## Preferred Language

Use:

```text
CommandReceipt
chunk_deltas
material_deltas
dev_event_summary
RenderFrame
VisibleCell
Perception Event Ledger
ColonyView
DevTruth
```

Avoid for Chunk 0:

```text
board_deltas
result_card
card UI
card dashboard
zone-only game
```

## Review Output

When reviewing, return:

1. corrections required;
2. corrected replacement text or files;
3. whether implementation is authorized;
4. remaining approval gates.

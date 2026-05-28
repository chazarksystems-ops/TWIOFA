# TWIOFA Development Spine

## Purpose

This folder is the repo-level coordination layer for splitting TWIOFA development into categories and playable slices without moving code prematurely.

It is intentionally docs-first.

This pass does not change gameplay code, Rust scaffold files, tests, Cargo configuration, reports, or archive material.

## Current Repo Target

Active repo for this pass:

```text
chazarksystems-ops/TWIOFA
```

Reference-only repo for now:

```text
chazarksystems-ops/TWIOFA-v0.01
```

Reason:

- `TWIOFA` contains the current verified Chunk 0 Rust scaffold and acceptance reports.
- `TWIOFA-v0.01` contains substrate-proof-first design contracts and anti-drift rules, but is documentation-first and not the current implementation repo.

## Current Pass

```text
PASS 1 — Docs-only category map and slice-development rules
```

Allowed changes in this pass:

```text
docs/development_spine/**
```

Forbidden changes in this pass:

```text
README.md
AGENTS.md
Cargo.toml
chunk0_rust_scaffold/**
spec_pack/**
docs/elevated_reasoning/**
docs/playable_slice_completion/**
agent_mapping_pass_01/**
archive/**
source_review/**
```

## Reading Order

Future agents should read these in order before proposing repo/category changes:

```text
1. README.md
2. AGENTS.md
3. CHUNK_0_PLAYABLE_SLICE_REPORT.md
4. docs/development_spine/README.md
5. docs/development_spine/REPO_CATEGORY_MAP.md
6. docs/development_spine/SLICE_DEVELOPMENT_RULES.md
7. docs/development_spine/AGENT_PREFLIGHT_TEMPLATE.md
8. docs/development_spine/AGENT_REPORT_TEMPLATE.md
9. docs/development_spine/DO_NOT_TOUCH_WITHOUT_APPROVAL.md
```

## Operating Principle

TWIOFA should be split into development categories, but implemented through playable slices.

A category explains ownership and intent.

A playable slice proves that multiple categories are working together in a way the player can feel.

Do not split the repo into discipline folders by moving files until the current implementation truth has been mapped and Chaz approves the next pass.

## Current Known Tension

The current public `TWIOFA` repo is validated around `chunk0_rust_scaffold`, `chunk0_harness`, and `chunk0_cli`.

Recent playability planning references a graphical `chunk0_game` target. This pass does not assume that target exists in GitHub. A future sync audit must verify whether `chunk0_game` exists locally, in another branch, or only in planning docs.

## Pass 1 Success Criteria

This pass succeeds if:

- repo categories are defined;
- existing repo areas are mapped without moving them;
- future playable slices have a standard contract shape;
- agent preflight/report templates exist;
- no implementation files are touched;
- no claim is made that the game has been reorganized yet.

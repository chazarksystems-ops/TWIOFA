# S00 — Repo Truth and Sync Audit

## Purpose

Before implementing graphical playability or splitting files physically, verify what exists in the active GitHub repo versus what exists in recent planning language.

This slice is required because the active GitHub repo currently validates CLI/harness Chunk 0, while recent playability planning references a graphical `chunk0_game` target that must be located or reconciled.

## Player-Visible Outcome

None directly. This is an audit slice.

Developer-visible outcome:

```text
Future agents know which repo, branch, binaries, docs, and implementation files are real before editing.
```

## Included Scope

```text
- Inspect root README.md
- Inspect AGENTS.md
- Inspect chunk0_rust_scaffold/Cargo.toml
- Search for chunk0_game
- Search for docs/playability if present
- Compare current GitHub repo against recent planning assumptions
- Produce a short sync report
```

## Excluded Scope

```text
- No implementation changes
- No folder moves
- No Cargo target additions
- No graphical game implementation
- No deletion of old docs
- No merging from TWIOFA-v0.01
```

## Allowed Files

```text
docs/development_spine/reports/S00_REPO_TRUTH_AND_SYNC_AUDIT_REPORT.md
```

## Forbidden Files

```text
README.md
AGENTS.md
chunk0_rust_scaffold/**
spec_pack/**
docs/elevated_reasoning/**
docs/playable_slice_completion/**
archive/**
source_review/**
```

## Required Docs to Read

```text
README.md
AGENTS.md
CHUNK_0_PLAYABLE_SLICE_REPORT.md
chunk0_rust_scaffold/Cargo.toml
docs/development_spine/README.md
docs/development_spine/SLICE_DEVELOPMENT_RULES.md
```

## Audit Questions

```text
1. Does chunk0_game exist in the active GitHub repo?
2. Does docs/playability exist in the active GitHub repo?
3. Are graphical playability docs local-only, missing, or in another repo/branch?
4. Are validation commands still aligned with current Cargo targets?
5. What is the next safe implementation pass after this audit?
```

## Validation

Docs-only validation:

```text
- no implementation files changed
- report lists evidence
- report identifies uncertainty honestly
- report does not claim gameplay improvement
```

## Acceptance Gate

S00 is accepted when Chaz can answer:

```text
Do we need to import/reconcile graphical playability docs before coding, or is the repo ready for the next implementation pass?
```

## Stop Conditions

Stop if:

```text
- required source docs are missing
- repo state contradicts the assigned task
- implementation changes become necessary
```

## Rollback

Delete only the S00 report file if the audit is invalid. Do not touch implementation files.

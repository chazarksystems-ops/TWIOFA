# Do Not Touch Without Approval

## Purpose

This file protects implementation truth and historical provenance from accidental cleanup, restructuring, or broad agent edits.

A category map is not permission to move files.

A slice contract is not permission to edit outside its allowed scope.

## Protected Paths

Do not modify, move, rename, or delete these without explicit Chaz approval and a dedicated migration plan:

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
CHUNK_0_PLAYABLE_SLICE_REPORT.md
CHUNK_0_IMPLEMENTATION_REPORT.md
CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md
BUILD_HANDOFF_V0_3_3.md
```

## Protected Concepts

Do not silently change these:

```text
Chunk 0 is a deterministic local scaffold.
Chunk 0 does not lock full-game canon.
Simulation determinism is mandatory.
No nondeterministic authoritative simulation behavior.
No full-game expansion without approval.
No license selection without approval.
No release/tag/public-demo language without approval.
No debug-grid acceptance as final visual direction.
No card-dashboard drift.
```

## Special Handling: TWIOFA-v0.01

`chazarksystems-ops/TWIOFA-v0.01` is reference-only for this pass.

Do not copy its substrate-proof rules into `TWIOFA` as implementation requirements unless Chaz explicitly chooses to merge that direction.

Allowed use:

```text
- compare project direction
- identify drift risks
- inform future branch planning
```

Forbidden use without approval:

```text
- replace current Chunk 0 scaffold target
- delete or deprecate current scaffold work
- start GPU/WebGPU implementation inside current scaffold
- declare old scaffold obsolete as a repo fact
```

## Migration Plan Requirement

Before any physical repo split, produce a migration plan that includes:

```text
1. exact files/folders to move
2. old path -> new path mapping
3. imports/references that must be updated
4. validation commands
5. rollback procedure
6. expected commit count
7. Chaz approval checkpoint
```

No physical split should happen in the same pass that proposes the split.

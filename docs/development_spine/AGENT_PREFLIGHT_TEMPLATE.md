# TWIOFA Agent Preflight Template

Agents must complete this before changing implementation-related files.

For docs-only tasks, fill it out in shortened form and explicitly state that no implementation files will be touched.

## Assigned Slice

```text
Slice ID:
Slice title:
Task type: docs-only / audit / implementation / validation / cleanup
```

## Goal

```text
What is the narrow goal of this pass?
What player-visible or developer-visible improvement should exist afterward?
```

## Repo Truth Checked

```text
Repo:
Branch:
Docs read:
Current implementation target:
Current known mismatch or uncertainty:
```

## Files Expected to Change

```text
- path/to/file
- path/to/file
```

## Files Forbidden to Change

```text
- path/to/file_or_folder
- path/to/file_or_folder
```

## Allowed Commands / Validation

```text
Commands planned:
Manual checks planned:
```

For Rust behavior changes, default validation is:

```sh
cd chunk0_rust_scaffold
cargo fmt --check
cargo check
cargo test
cargo run --bin chunk0_harness -- run-corpus
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
cargo run --bin chunk0_cli -- run-script smoke
```

## Acceptance Criteria

```text
- Criterion 1
- Criterion 2
- Criterion 3
```

## Drift Risks

```text
What could this pass accidentally overbuild, mislabel, or canonize?
```

## Stop Conditions

```text
Stop if:
- required file is missing
- validation fails
- scope expands beyond allowed files
- repo truth contradicts the task assumptions
- graphical target is assumed but not found
```

## Rollback / Recovery Note

```text
How should this pass be reverted or abandoned if it fails?
```

## Final Pre-Edit Statement

```text
I will only change the listed allowed files. I will not touch forbidden paths. I will report all validation honestly and will not claim success if required checks fail.
```

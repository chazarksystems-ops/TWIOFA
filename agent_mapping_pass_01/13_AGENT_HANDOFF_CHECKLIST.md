# 13 — Agent Handoff Checklist

Any coding agent must read this document before beginning implementation of Chunk 0. The agent must explicitly answer every item before writing any source code.

---

## Pre-Implementation Declarations

Before writing any code, the agent must state:

### 1. Files Expected to Change

```
List every file the agent plans to create or modify.
Example:
- src/cell.rs (NEW)
- src/chunk.rs (NEW)
- src/sim.rs (NEW)
- ...
```

### 2. Files Forbidden to Change

```
The following files must NOT be modified during implementation:

- spec_pack/*.md (all 14 spec files)
- source_review/*.txt (all 15 historical files)
- README.md (workspace root)
- SOURCE_REVIEW_NOTES.md
- SPEC_V0_3_2_CORRECTION_REPORT.md
- skills/twiofa_chunk0_spec_hardener/SKILL.md
- agent_mapping_pass_01/*.md (all mapping documents)

The agent must not rewrite spec files as design changes.
The agent must not add lore, canon, or game design documents.
The agent must not modify the old micro-yard card-dashboard expansion.
```

### 3. Exact Commands to Run

```
List every build, test, and run command.
Example:
- cargo build
- cargo test
- cargo run (to launch visualizer)
- cargo test --test determinism_test
```

### 4. Test Plan

```
List every C0-TEST-* the agent will implement and run.
State which fixture each test uses.
State expected pass/fail criteria.
State how golden hash will be generated (only after first deterministic run).
```

### 5. Rollback Plan

```
State how to undo all changes if implementation fails.
Example:
- git revert to pre-implementation commit
- delete src/ directory
- verify no spec files were modified
```

### 6. No Card-Dashboard Regression Proof

```
State how the agent will prove no card-dashboard regression.
Must include:
- Screenshot or description showing 128×128 grid as central element
- Confirmation that no card/log/menu is the main interaction surface
- C0-TEST-UI-1 result
- C0-TEST-UI-2 result
```

### 7. Deterministic Replay Proof

```
State how the agent will prove deterministic replay.
Must include:
- C0-TEST-DET-1 result (same commands 3× → same hash)
- C0-TEST-DET-2 result (UI changes don't affect hash)
- Hash values from each run
```

### 8. WorldTruth Does Not Leak to Default UI Proof

```
State how the agent will prove WorldTruth doesn't leak.
Must include:
- C0-TEST-PERC-1 result (hidden truth absent in ColonyView)
- C0-TEST-PERC-2 result (bitter/yellow, not Sourback)
- C0-TEST-UI-3 result (DevTruth toggle works correctly)
- Grep for "Sourback" in ColonyView output strings
```

---

## Stop Triggers

The agent **MUST IMMEDIATELY STOP and request Chaz approval** if any of the following conditions arise:

| Trigger | Why |
|---|---|
| Asked to add procedural yard/world generation | Scope violation: spec `08` ban |
| Asked to use Unreal Engine | Scope violation: spec `08` ban |
| Asked to add GPU compute or GPU-owned simulation | Scope violation: spec `06`/`08` ban |
| Asked to add A* pathfinding | Scope violation: spec `04`/`08` ban; greedy movement only |
| Asked to make a card/log UI the main surface | Card-dashboard regression: spec `06` anti-card rule |
| Asked to implement full colony AI | Scope violation: spec `08` ban on full ant AI |
| Hidden truth must be exposed to default player UI | Epistemic boundary violation: spec `05`/`06` |
| Asked to add multiplayer | Scope violation: spec `08` ban |
| Asked to add doctrine system or Deep Queen layer | Scope violation: spec `08` ban |
| Asked to add spider/centipede creature arcs | Scope violation: spec `08` ban |
| Asked to add full food economy | Scope violation: spec `08` ban |
| Asked to add another UI framework (beyond approved visualizer) | Scope violation: spec `08` stop trigger |
| Asked to add full Field Notes archive UI | Scope violation: spec `05`/`08`; deferred per D7 |
| Asked to add new canon or lore | Scope violation: spec `08` stop trigger |
| Feature proposed that doesn't prove any of the 11 scope-gate topics | Scope violation: spec `08` scope gate |
| Tick order needs to change | Must revise spec and tests first (spec `03` L22) |
| Build-readiness gate has not been approved | Implementation prohibition: spec `09`/`12` |

---

## Implementation Report Requirement

Upon completion, the agent **must** generate:

```
CHUNK_0_IMPLEMENTATION_REPORT.md
```

Required sections (from spec `09` L98–111):

1. Files Changed
2. How to Run
3. Controls
4. Validation Commands Run
5. C0-TEST Results
6. Determinism Proof / Hashes
7. Visualizer Check
8. Anti-Card Compliance Check
9. Known Deviations
10. Known Limitations
11. Next Recommendations
12. Current git status

---

## Post-Implementation Review

After submitting the implementation report:

1. Chaz reviews the report
2. Chaz runs the manual visual smoke path (spec `07` L39–48)
3. Chaz confirms anti-card compliance
4. Chaz confirms truth boundary compliance
5. Chaz confirms determinism proof
6. If any check fails, agent must fix or rollback

---

## Agent Acknowledgment

Before starting implementation, the agent must output:

```
I have read the Agent Handoff Checklist.
Build-readiness gate status: [APPROVED / NOT APPROVED]
I will implement the following files: [list]
I will not modify: [list]
My test plan covers: [list of C0-TEST IDs]
My stop triggers are active.
```

If the build-readiness gate is NOT APPROVED, the agent must stop immediately.

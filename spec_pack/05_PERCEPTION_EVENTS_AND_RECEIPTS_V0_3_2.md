# 05 — Perception, Events, and Receipts

## Core Split

```text
WorldTruth = what physically exists and what the simulation knows
ColonyPerception = what the colony has observed, inferred, misread, or later reframed
```

## WorldTruth Includes

```text
ChunkCells / Vec<Cell>
ant group state
tick index
seed and deterministic RNG state if used
hidden semantic meanings of residue/hazards
command history if required for replay
```

The `Vec<Cell>` is the main material substrate, not the entire WorldTruth.

## ColonyPerception Includes

```text
observed facts
suspected hazards
confidence states
interpretations
misreads/reframes
assignment policy such as Avoid
```

For Chunk 0, this is called the **Perception Event Ledger**.

Full Field Notes UI is deferred. The Perception Event Ledger must remain compatible with later Field Notes.

## Language Guardrail

Before explicit earned observation:

```text
Allowed: bitter/yellow residue
Forbidden: Sourback defensive secretion
Forbidden: Sourback Caterpillar path
```

After explicit carcass-edge / Sourback observation event:

```text
Allowed: Sourback-associated path
Allowed: residue is associated with Sourback pressure
```

## Event Schema

Events must have stable ordering and stable IDs.

Minimum event types:

| Event | Required fields | Purpose |
|---|---|---|
| `PhysicalFact` | id, tick, desc, coords | Objective colony-accessible observation. |
| `ColonyInterpretation` | id, tick, claim, relates_to | Subjective model. |
| `WorkerLoss` | id, tick, lost, coords | Physical consequence. |
| `Correction` | id, tick, claim, corrects, evidence | Replaces old interpretation. |
| `Reframe` | id, tick, claim, relates_to, evidence | Keeps old observation true but incomplete. |
| `CommandFailed` | id, tick, reason, command | Debug + player-facing failure. |
| `CellChanged` | id, tick, coord, from, to | Material delta event. |
| `CollapseOccurred` | id, tick, coords | Structural change event. |
| `CarcassHarvested` | id, tick, coord, food_units | Physical food event. |
| `AntGroupSlowed` | id, tick, coord, reason | Residue/moisture slowdown. |

## Event-to-Perception Mapping

| WorldTruth / sim event | ColonyPerception output |
|---|---|
| ant enters `SourbackBitter` before confirmation | `PhysicalFact(desc="Yellow bitter residue encountered")` |
| worker loss near residue | `WorkerLoss` + interpretation: route unsafe |
| carcass observed | `PhysicalFact(desc="Carcass edge observed")` |
| Sourback-associated residue confirmed | `Correction` or `Reframe` linking earlier residue facts |
| tunnel collapses | `PhysicalFact(desc="Tunnel segment collapsed")` |
| water erases scent | `PhysicalFact(desc="Scent weakened near wet seam")` if observed |

## CommandReceipt Contract

After resolving a command, Rust returns a `CommandReceipt`.

**Naming note (v0.3.3):** `CommandReceipt` is the conceptual Rust/domain type name. `command_receipt` (snake_case) is the field/token name used in serialized receipts, JSON examples, and API surfaces where Rust naming conventions apply. This distinction prevents drift without dictating implementation details.

Required fields:

```text
command_id
tick_start
tick_end
chunk_hash_before
chunk_hash_after
chunk_deltas
perception_updates
dev_event_summary
debug_stats
```

Use these names:

```text
chunk_deltas
material_deltas
command_receipt
dev_event_summary
```

Avoid these names for Chunk 0:

```text
board_deltas
result_card
card
card-driven UI
```

Example conceptual receipt:

```json
{
  "command_id": 7,
  "tick_start": 40,
  "tick_end": 45,
  "chunk_hash_before": "hash_before",
  "chunk_hash_after": "hash_after",
  "chunk_deltas": [
    { "type": "CellMaterial", "x": 55, "y": 95, "from": "Soil", "to": "Tunnel" }
  ],
  "perception_updates": [
    { "type": "PhysicalFact", "desc": "Yellow bitter residue encountered", "coords": [[90, 30]] }
  ],
  "dev_event_summary": "Digging opened one cell. The residue remains unexplained.",
  "debug_stats": { "ticks_advanced": 5 }
}
```

The `dev_event_summary` may be shown in a small debug event panel. It must not become the main gameplay surface.

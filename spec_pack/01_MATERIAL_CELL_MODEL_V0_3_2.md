# 01 — Exact Material and Cell Model

## Cell Schema

The chunk uses a flat 1D array of 16,384 cells:

```text
width = 128
height = 128
index = y * width + x
```

Cell target:

```text
small
copyable
no heap allocation
stable for deterministic hashing via canonical serialization
```

## Expected First-Pass Cell Fields

| Field | Type class | Range | Default | Hash? | Meaning |
|---|---|---:|---|---:|---|
| `material` | u8-compatible enum | 0..N | `Soil` except explicit bands | Yes | Physical material identity |
| `moisture` | `u8` | 0..255 | 0 | Yes | Local wetness |
| `scent_home` | `u8` | 0..255 | 0 | Yes | Return-to-nest scent |
| `scent_food` | `u8` | 0..255 | 0 | Yes | Food/carcass scent |
| `residue` | u8-compatible enum | 0..N | `None` | Yes | Physical residue/hazard marker |
| `support` | `u8` | 0..255 | material-dependent | Yes | Local structural support |
| `flags` | `u8` bitmask | 0..255 | 0 | Yes | Recently dug/collapsed/observed markers |

Do not claim exact Rust byte size unless verified by a unit test using `std::mem::size_of::<Cell>()`.

Forbidden inside `Cell`:

```text
String
Vec
Box
HashMap
Rc / Arc
any heap-owned field
```

## Material Enum

Minimum materials:

```text
Air
Soil
LooseSoil
Tunnel
Water
Carcass
Root
Stone
NestWall
```

## Material Semantics Table

| Material | Traversable by AntGroup? | Diggable? | Support default | Holds moisture? | Holds scent? | Harvestable? | Render role |
|---|---:|---:|---:|---:|---:|---:|---|
| Air | Yes | No | 0 | Fast pass-through | Weak | No | Empty / above ground / open space |
| Soil | No | Yes | 200 | Slow | No | No | Main substrate |
| LooseSoil | No | Yes | 60 | Slow | No | No | Collapse-risk material |
| Tunnel | Yes | No | 0 | Fast pass-through | Strong | No | Dug ant space |
| Water | Slow / costly | No | 0 | Source | Erases | No | Flowing/pooling water |
| Carcass | No | Edge-harvest only | 100 | No | Food-source adjacent | Yes | Physical food objective |
| Root | No | No | 255 | Blocks | No | No | Structural anchor/obstacle |
| Stone | No | No | 255 | Blocks | No | No | Boundary / unbreakable |
| NestWall | No | No | 255 | Blocks | Home scent nearby | No | Protected nest structure |

Correction: `NestWall` is not ant-traversable. The nest chamber itself is represented by `Tunnel`/open traversable cells.

**Tunnel support note (v0.3.3):** Tunnel support default = 0 is intentional because Tunnel represents open space / absence of supporting material. This is not an error. Future agents and passes must not "fix" or change this value back to 255 (which would incorrectly model tunnels as indestructible structural anchors). The value 0 correctly allows collapse risk above open tunnels.

## Residue Enum

Minimum residues:

```text
None
SourbackBitter
Rot          // placeholder only, no full system yet
Alarm        // placeholder only, no full system yet
```

Chunk 0 requires only `SourbackBitter` behavior.

## Flags

Minimum bit flags:

| Flag | Meaning |
|---|---|
| `RECENTLY_DUG` | Cell was changed by digging recently. |
| `RECENTLY_COLLAPSED` | Cell moved or was filled by collapse recently. |
| `OBSERVED_THIS_TICK` | Optional event/inspection helper; may be visual-only if excluded from hash. |
| `HARVESTED` | Optional, only if needed; otherwise carcass cell conversion is enough. |

Any flag included in the canonical cell state must be included in the chunk hash. Visual-only flags must be stored outside `Cell`.

## Material Interaction Table

| Interaction | Chunk 0 outcome |
|---|---|
| Water + scent | `Water` cell sets both scent values to 0 each tick. Wet non-water cells reduce scent by an extra fixed amount. |
| LooseSoil + open below | If `support < 100` and below is `Air` or `Tunnel`, `LooseSoil` swaps downward. |
| Digging + support | Converting `Soil`/`LooseSoil` to `Tunnel` reduces support of cardinal neighbors by 50. |
| Carcass + harvest | One accessible `Carcass` cell converts to `Air` and emits one food harvest event. No per-cell mass in Chunk 0. |
| SourbackBitter + Scout | Emits bitter/yellow residue observation; does not reveal hidden Sourback meaning unless carcass-edge observation permits it. |
| SourbackBitter + Forage | Deterministically slows movement and may emit worker-loss event under locked rule. |
| Moisture + support | Wet `LooseSoil` may lose support at a fixed deterministic rate. |

## Budget Constraints

- 128x128 = 16,384 cells.
- One full tick should run comfortably on a single CPU thread.
- No per-tick heap churn should be required for the core material array after initialization.
- Packed layers may be considered later; `Vec<Cell>` is the first-pass contract.

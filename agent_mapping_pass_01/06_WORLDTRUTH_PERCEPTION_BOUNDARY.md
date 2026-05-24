# 06 — WorldTruth / Perception Boundary Map

Deep mapping of the epistemic boundary between what physically exists and what the colony is allowed to know.

---

## WorldTruth Fields

Everything the simulation knows and owns.

| Field / Component | Scope | Source |
|---|---|---|
| `Vec<Cell>` (16,384 cells) | Main material substrate | `01`, `05` |
| Cell.material | Physical material identity (Material enum) | `01` |
| Cell.moisture | Local wetness (u8, 0–255) | `01` |
| Cell.scent_home | Return-to-nest scent intensity (u8, 0–255) | `01` |
| Cell.scent_food | Food/carcass scent intensity (u8, 0–255) | `01` |
| Cell.residue | Physical residue identity (Residue enum, e.g., SourbackBitter) | `01` |
| Cell.support | Structural support value (u8, 0–255) | `01` |
| Cell.flags | Bit flags (RECENTLY_DUG, RECENTLY_COLLAPSED, etc.) | `01` |
| Ant group state | id, pos, workers, task, food_carried, fatigue, confidence, memory | `04` |
| Tick index | Current simulation tick | `03`, `05` |
| Seed / RNG state | Deterministic seed (stored, unused in v0.3.2 layout) | `02`, `05` |
| Hidden semantic meanings | e.g., SourbackBitter = Sourback Caterpillar defensive secretion | `05` |
| Command history | For replay (if required) | `05` |

---

## RenderFrame Fields

Derived from WorldTruth. Sent to the visualizer.

| Field | Contents | Truth Level |
|---|---|---|
| `tick_index` | Current tick number | Neutral (not hidden) |
| `chunk_hash` | Canonical hash of simulation state | Debug/verification data |
| `visible_cells` | Array/buffer of VisibleCell or pixel colors | Colony-safe by default |
| `overlay_mode` | Currently selected overlay (Material, Moisture, Scent, Residue, Support, Perception) | UI state |
| `chunk_deltas` | List of cell changes since last frame | Colony-safe (material category only, not hidden semantics) |
| `perception_markers` | Colony-known observations for rendering | Colony-safe |
| `debug_stats` | Tick count, performance data | Debug-only |

**Key rule**: RenderFrame must **exclude** hidden semantic truth unless DevTruth mode is explicitly enabled.

---

## VisibleCell Fields (ColonyView Default)

What the default player-facing view may include per cell.

| Field | Allowed | Example |
|---|---|---|
| `coord` | Yes | (55, 95) |
| `visible_material_category` | Yes | "Tunnel", "Soil", "Water" |
| `visible_color_id` / palette index | Yes | Color mapped from material category |
| `visible_overlay_value` | Yes (for selected overlay) | Moisture intensity as gradient |
| `known_perception_marker` | Yes (if observed) | "bitter/yellow residue" |
| `recent_delta_marker` | Yes | "recently dug" |

---

## VisibleCell Forbidden Fields

What must **never** appear in default ColonyView rendering.

| Forbidden Field | Reason | Source |
|---|---|---|
| Hidden residue semantic meaning | Sourback identity is not colony-accessible before earned observation | `06` L64–70 |
| Hidden hazard identity | Colony doesn't know what the hazard *is*, only what it *looks like* | `06` L65 |
| Exact hidden collapse probability | Colony can see "unstable ground" signs, not raw support numbers | `06` L66 |
| Unearned Sourback labels | "Sourback" cannot appear until carcass-edge observation event | `05` L40–53 |
| Raw debug-only fields | support values, exact scent numbers, raw flags | `06` L67 |

---

## DevTruth Fields

What the developer debug mode may expose.

| Field | Available in DevTruth | Not in ColonyView |
|---|---|---|
| Cell.material (raw enum) | Yes | Only category name |
| Cell.moisture (exact u8) | Yes | Only visual gradient |
| Cell.scent_home (exact u8) | Yes | Only "scent detected" indicator |
| Cell.scent_food (exact u8) | Yes | Only "scent detected" indicator |
| Cell.residue (raw enum, e.g., `SourbackBitter`) | Yes | Only "bitter/yellow residue" |
| Cell.support (exact u8) | Yes | Not shown |
| Cell.flags (raw bitmask) | Yes | Only derived markers |
| Hidden semantic meaning | Yes | Never |

---

## ColonyView Fields

What the colony perception system provides.

| Field | Contents | Source |
|---|---|---|
| Observed facts | Physical observations as natural language | `05` L22–26 |
| Suspected hazards | Colony-inferred danger markers | `05` L27 |
| Confidence states | Scalar or band indicating colony trust in a route/area | `04` L138–155 |
| Interpretations | Subjective models based on evidence | `05` L28 |
| Misreads / reframes | Old interpretations kept as true-but-incomplete | `05` L29–30 |
| Assignment policy | Avoid zones, confidence/risk markers | `04` L96–117 |

---

## Truth Boundary Translation Table

| WorldTruth (Hidden) | ColonyView (Player-Facing) | Trigger for Upgrade |
|---|---|---|
| `Residue::SourbackBitter` | "Yellow bitter residue" | Default — before any observation |
| `Residue::SourbackBitter` (with confirmed source) | "Sourback-associated path" | After carcass-edge / Sourback observation event |
| `Cell.support = 45` (exact value) | "Unstable ground" or "collapse risk" (visual cue) | Never shown as number in ColonyView |
| `Cell.moisture = 200` (exact value) | Moisture intensity gradient / "wet area" | Shown as visual overlay, not raw number |
| `Cell.scent_home = 180` | "Strong home trail" or gradient color | Not raw number |
| `Cell.scent_food = 50` | "Faint food trail" or gradient color | Not raw number |
| Worker loss cause: Sourback toxicity | "Workers lost in bitter area" | Physical consequence event |
| Collapse cause: support < 100 + open below | "Tunnel collapsed" | Physical fact event |
| Water erasing scent (mechanism) | "Scent weakened near wet seam" | Only if ant group observes the area |
| Carcass composition (internal) | "Carcass edge observed" → "Food source" | Physical observation event |

---

## What the Default Visualizer May Receive

| Data | Allowed? | Notes |
|---|---|---|
| Material category for coloring | ✅ Yes | "Soil", "Tunnel", "Water" — not internal enum value labels |
| Overlay intensity values | ✅ Yes | Moisture/scent as gradients |
| Perception markers | ✅ Yes | Colony-known observations |
| Delta markers (recently dug) | ✅ Yes | Visual feedback |
| Chunk hash | ✅ Yes | For debug display |
| Tick index | ✅ Yes | For debug display |
| Raw Cell struct | ❌ No | Must go through RenderFrame |
| Residue enum name | ❌ No | Only guardrailed text |
| Hidden semantic meaning | ❌ No | Never in default mode |
| Support values | ❌ No | Debug/DevTruth only |

---

## What DevTruth May Request

| Data | Allowed? | Notes |
|---|---|---|
| All Cell fields (raw) | ✅ Yes | Full struct dump |
| Hidden semantic meanings | ✅ Yes | "SourbackBitter = Sourback defensive secretion" |
| Support numbers | ✅ Yes | Exact u8 values |
| Ant group internal state | ✅ Yes | All fields |
| Chunk hash | ✅ Yes | Already available |
| Full event log | ✅ Yes | Including internal events |

---

## What Must NEVER Appear in Default Player-Facing UI

| Item | Reason |
|---|---|
| "Sourback" label before earned observation | Language guardrail violation |
| "Sourback Caterpillar path" before earned observation | Canon leak |
| "Sourback defensive secretion" | Hidden WorldTruth semantic |
| Raw `support` numeric values | Internal simulation data |
| Raw `moisture` / `scent` exact numbers | Should be visual gradients, not numbers |
| Raw `flags` bitmask | Internal |
| Cell struct field names | Implementation detail |
| Hidden command log entries | Internal replay data |
| Collapse probability calculations | Internal |

---

## Example: SourbackBitter Epistemic Journey

```
TICK 0: WorldTruth has SourbackBitter residue at (80..116, 28..36)
        ColonyView: nothing — area not yet observed

TICK N: Ant group scouts near residue path
        WorldTruth event: AntEntersSourbackBitter(coord)
        ColonyPerception: PhysicalFact(desc="Yellow bitter residue encountered")
        VisibleCell: known_perception_marker = "bitter/yellow residue"
        ❌ NOT: "Sourback residue"

TICK N+M: Ant group forages through residue, workers lost
        WorldTruth event: WorkerLoss(lost=3, cause=SourbackBitter)
        ColonyPerception: WorkerLoss + ColonyInterpretation(claim="Route unsafe — bitter substance")
        ❌ NOT: "Sourback toxicity"

TICK N+M+K: Ant group observes carcass edge near residue
        WorldTruth event: CarcassObserved + SourcebackAssociationEarned
        ColonyPerception: Correction/Reframe linking bitter residue to Sourback pressure
        VisibleCell: known_perception_marker upgrades to "Sourback-associated path"
        ✅ NOW ALLOWED: "Sourback" label
```

---

## Boundary Enforcement Strategy

1. **Structural**: `RenderFrame` / `VisibleCell` types physically cannot contain hidden fields.
2. **API**: The visualizer receives only `RenderFrame`, never raw `WorldTruth`.
3. **Language**: Event descriptions use guardrailed text from a translation table.
4. **Testing**: C0-TEST-PERC-1 verifies absence of hidden truth; C0-TEST-UI-3 verifies DevTruth toggle works.
5. **Review**: Manual smoke path step 8 explicitly checks residue labeling.

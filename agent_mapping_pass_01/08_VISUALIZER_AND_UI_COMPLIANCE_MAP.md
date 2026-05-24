# 08 — Visualizer and UI Compliance Map

Comprehensive mapping of the visualizer contract, rendering rules, and compliance requirements.

---

## Central 128×128 Grid/Canvas Requirement

| Attribute | Spec Rule | Source |
|---|---|---|
| Main surface | Material grid/canvas is the main surface | `06` L4 |
| Grid size | 128×128 cells | `00` L31, `02` L5 |
| Central element | Grid must be the central, dominant visible element | `06` L132–138 |
| Debug panels | Support the grid; must never become the game | `06` L7–9 |
| Pixel rendering | Each cell maps to at least one pixel/tile on screen | `06` L21 |
| Scaling | Not specified — implementer may choose pixel-perfect scaling (e.g., 4× → 512×512 window) | — |

---

## Overlay Modes

| Overlay | Shows | Default Truth Exposure | Source |
|---|---|---|---|
| Material | Visible material category (color-coded) | Colony-safe | `06` L113 |
| Moisture | Moisture intensity gradient | Colony-safe; DevTruth for exact values | `06` L114 |
| Scent | Home/food scent intensity | Colony-safe if scent observed/accessible | `06` L115 |
| Residue | Known bitter/yellow residue markers | Colony-safe for observed cells only | `06` L116 |
| Support | Collapse/support debug values | DevTruth/debug-only by default | `06` L117 |
| Perception | Known/suspected/misread markers | Colony-safe | `06` L118 |

**Note**: Support overlay is debug-only by default. Showing exact support numbers in ColonyView would violate the epistemic boundary.

---

## RenderFrame Contract

### Required Fields

| Field | Type (conceptual) | Purpose | Source |
|---|---|---|---|
| `tick_index` | u64 | Current simulation tick | `06` L39 |
| `chunk_hash` | String/Hash | Canonical hash for verification | `06` L40 |
| `visible_cells` | Buffer/Array | Pixel/color data or VisibleCell array | `06` L41 |
| `overlay_mode` | Enum | Currently selected overlay | `06` L42 |
| `chunk_deltas` | Vec | Cell changes since last frame | `06` L43 |
| `perception_markers` | Vec | Colony-known observation markers | `06` L44 |
| `debug_stats` | Struct | Performance/tick data | `06` L45 |

### Excluded from RenderFrame (Default Mode)

| Excluded Data | Reason | Source |
|---|---|---|
| Hidden residue semantic meaning | Epistemic boundary | `06` L48, L64 |
| Hidden hazard identity | Epistemic boundary | `06` L65 |
| Exact collapse probability | Debug data | `06` L66 |
| Unearned Sourback labels | Language guardrail | `06` L67 |
| Raw debug-only fields | Not player-facing | `06` L68 |

---

## Inspection Behavior

### ColonyView Mode (Default)

| Interaction | Shows | Source |
|---|---|---|
| Hover/click on cell | Observed material category | `06` L80 |
| | Observed moisture/scent/residue signs | `06` L81 |
| | Known or suspected interpretation | `06` L82 |
| | Confidence band | `06` L83 |
| | Related perception events | `06` L84 |

### DevTruth Mode (Explicit Toggle)

| Interaction | Shows | Source |
|---|---|---|
| Hover/click on cell | All raw Cell fields | `06` L96–103 |
| | Hidden semantic meanings | `06` L103 |
| | Not player-facing | `06` L106 |

---

## Debug Panels

| Panel Type | Purpose | Rule | Source |
|---|---|---|---|
| Debug event panel | Shows `dev_event_summary` | Supports grid; must not dominate | `05` L141 |
| Chunk hash readout | Shows current hash | Debug tool | `08` L57 |
| Command controls | Buttons/keys for commands | Allowed | `08` L54 |
| Overlay toggles | Switch between overlay modes | Allowed | `08` L53 |
| Cell inspector | Shows cell data in selected mode | Allowed | `08` L54 |

---

## Anti-Card-Dashboard Rule

### Build FAILS if:

| Condition | Source |
|---|---|
| Central visual element is a list of cards/logs/buttons | `06` L126 |
| Main interaction is a menu rather than the chunk grid | `06` L127 |
| Material changes are not visible on the grid | `06` L128 |

### Build PASSES only if:

| Condition | Source |
|---|---|
| 128×128 material grid/canvas is visible | `06` L133 |
| Commands visibly alter cells | `06` L134 |
| Overlays change what is drawn | `06` L135 |
| Inspection is tied to cells/regions | `06` L136 |

---

## Acceptable Tauri/Canvas Use

| Use | Allowed? | Condition | Source |
|---|---|---|---|
| HTML Canvas rendering 128×128 grid | ✅ Yes | Must be central element | `06` L21 |
| HTML/DOM for command buttons | ✅ Yes | Limited to controls | `06` L22 |
| HTML/DOM for debug panels | ✅ Yes | Limited to debug | `06` L22 |
| HTML/DOM for overlay toggles | ✅ Yes | Limited to controls | `06` L22 |
| HTML/DOM for cell inspector panel | ✅ Yes | Limited to inspection | `06` L22 |
| HTML/DOM for card-dashboard gameplay | ❌ No | Explicitly banned | `06` L24 |
| HTML/DOM as main game board | ❌ No | Canvas must be central | `06` L21 |
| Tauri IPC for sim communication | ✅ Yes | Necessary for Rust↔JS bridge | `09` L73 |

---

## Fallback Visualizer Options

| Option | Description | When to Use | Source |
|---|---|---|---|
| `minifb` | Software-rendered pixel buffer; trivial API | If Tauri becomes distracting or overhead is too high | `06` L29 |
| `pixels` | GPU-rendered via wgpu; provides `&mut [u8]` RGBA buffer | If GPU rendering needed but Tauri still unwanted | `06` L29 |

**Note on `pixels` and `wgpu`**: The `pixels` crate uses `wgpu` internally for rendering pixels to the screen. This is allowed. The ban is on custom GPU compute and GPU-owned simulation logic, not on transitive rendering internals (`06` L165).

---

## Explicit Failure Examples

These are examples of builds that would fail the anti-card compliance test:

| Failure Example | Why It Fails |
|---|---|
| Central screen is a scrollable log of event cards | Central element is cards, not grid |
| Grid is a small thumbnail in a sidebar; main area is command menus | Grid is not central |
| Food counter increments but carcass cells don't change on grid | Material changes not visible |
| Overlay buttons exist but grid doesn't update when overlay changes | Overlays don't change rendering |
| Cell click opens a full-screen card with stats | Inspection becomes card UI |
| No grid visible; only a text terminal showing simulation output | No material grid at all |
| Grid exists but is 32×32 or wrong size | Grid is not 128×128 |
| Debug event panel takes up 60% of screen | Debug panel dominates |

---

## Compliance Checklist

A future implementation must pass ALL of these checks:

### Grid Presence
- [ ] 128×128 material grid/canvas is rendered
- [ ] Grid is the central, largest visible element
- [ ] Grid occupies majority of screen area
- [ ] Each cell is individually distinguishable (at least 1 pixel per cell)

### Grid Interaction
- [ ] Issuing a dig command visibly changes cells on the grid
- [ ] Collapse visibly changes cells on the grid
- [ ] Water movement is visible on the grid
- [ ] Carcass harvest removes cells from the grid
- [ ] Scent changes are visible via overlay
- [ ] Moisture changes are visible via overlay
- [ ] Residue markers are visible via overlay

### Overlay System
- [ ] Material overlay works
- [ ] Moisture overlay works
- [ ] Scent overlay works
- [ ] Residue overlay works
- [ ] Support overlay works (DevTruth-only by default)
- [ ] Perception overlay works
- [ ] Switching overlays changes grid rendering

### Inspection
- [ ] Clicking/hovering a cell shows ColonyView data
- [ ] ColonyView hides hidden semantic truth
- [ ] DevTruth toggle shows raw Cell fields
- [ ] DevTruth toggle is explicitly labeled as debug mode

### Debug Panels
- [ ] Debug event panel exists
- [ ] Debug event panel does NOT dominate the UI
- [ ] Debug event panel updates after grid changes, not before
- [ ] Chunk hash is displayed
- [ ] Tick index is displayed

### Anti-Card Compliance
- [ ] No card-based layout as main surface
- [ ] No scrollable event log as main surface
- [ ] No menu system as main interaction surface
- [ ] No text-only simulation output
- [ ] Grid changes are the primary feedback, not event text

### Truth Boundary
- [ ] Default view does not show "Sourback" before earned observation
- [ ] Default view does not show raw support numbers
- [ ] Default view does not show raw Cell struct fields
- [ ] DevTruth toggle correctly reveals hidden truth
- [ ] RenderFrame struct does not contain hidden semantic data

### Dependency Compliance
- [ ] Tauri (if used) is only for canvas + controls
- [ ] No Bevy, Unreal, or other banned framework
- [ ] No custom GPU compute shader
- [ ] `pixels`/`wgpu` used only for transitive rendering (if applicable)

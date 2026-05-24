# 07 — Simulation Rule Map

Every simulation rule from spec `03` and `04`, with inputs, outputs, deterministic order, pseudocode references, precision gaps, edge cases, and required tests.

---

## 1. Digging

| Attribute | Value |
|---|---|
| **Inputs** | Ant group position; task = Dig; target cell coordinates |
| **Outputs** | Target cell: material → Tunnel, flags |= RECENTLY_DUG, support = 0; cardinal neighbors: support -= 50 (saturating); CellDug event emitted |
| **Deterministic order** | Single target cell, then 4 cardinal neighbors in fixed order |
| **Pseudocode section** | Spec `03` L78–90 |
| **Missing precision** | Cardinal neighbor order not specified (up/right/down/left assumed from harvest rule but not stated for digging). What if a neighbor is out of bounds? Saturating_sub handles negatives but boundary cells need clamping. |
| **Edge cases** | Digging at boundary (x=1 → neighbor at x=0 is boundary stone); digging next to Root/Stone (support already 255, subtract 50 = 205 — valid?); digging LooseSoil that is about to collapse in same tick |
| **Required tests** | C0-TEST-MAT-1 (basic dig), additional: dig at boundary, dig next to collapse-risk area |

---

## 2. Collapse

| Attribute | Value |
|---|---|
| **Inputs** | All cells in chunk |
| **Outputs** | LooseSoil cells swap with Air/Tunnel below; RECENTLY_COLLAPSED set on moved cell; CollapseOccurred event |
| **Deterministic order** | y from 126 down to 0; x from 0 to 127. Bottom-up ensures falling cells settle before upper cells check. |
| **Pseudocode section** | Spec `03` L94–108 |
| **Missing precision** | Can a cell collapse multiple rows in one tick? Pseudocode shows single swap, but after swap the now-lower LooseSoil could be checked again if loop continues downward (it won't — loop goes upward). Does the swapped Air/Tunnel cell above also get RECENTLY_COLLAPSED? Pseudocode says "moved_cell" gets the flag, which is the cell that moved down. |
| **Edge cases** | Chain collapse: LooseSoil above LooseSoil above Air — only bottom one falls per tick. LooseSoil at y=126 above stone floor at y=127 — cannot collapse. Ant group on the cell that swaps — spec doesn't address this. |
| **Required tests** | C0-TEST-MAT-2 (basic collapse), additional: chain collapse across multiple ticks, collapse into tunnel occupied by ant group |

> **Pre-v0.3.3 finding (historical)**: Ant group on the cell that swaps — spec doesn't address this. ... **ASSUMPTION (conservative)**: ... pushed or the collapse is blocked. ... **Flagged for Chaz review.**
> **v0.3.3 resolution**: Material swap still occurs (world-truth deterministic). If destination cell == AntGroup pos, AntGroup receives the *existing* worker-loss consequence (min(3,workers) lost, confidence -=16 clamped [0,255]), WorkerLoss event emitted, included in receipt/perception. No health/animation/combat systems invented. See spec_pack/03 L110-116 ("Collapse + AntGroup interaction (v0.3.3...)"). **RESOLVED IN v0.3.3.**

---

## 3. Water Flow

| Attribute | Value |
|---|---|
| **Inputs** | All water cells, processed bottom-up |
| **Outputs** | Water cell swaps with destination; moisture carried with water |
| **Deterministic order** | Bottom-up (same as collapse). For each water cell: try down, down-left, down-right, left, right. First valid move taken. |
| **Pseudocode section** | Spec `03` L112–127 |
| **Missing precision** | What is the iteration order for water cells at the same y? Left-to-right (x=0..127) is implied by "for each water cell in bottom-up order" but not stated. Does "bottom-up" mean y from 127 down to 0 or y from 126 down to 0 (since 127 is stone)? Can two water cells try to move into the same destination? (First one wins due to in-place update.) |
| **Edge cases** | Water at boundary (x=0, cannot go left or down-left). Water adjacent to water (both try to move to same empty cell). Water on top of water (cannot move down). All paths blocked (water stays). Water carrying moisture value 255 — does the destination cell inherit it? |
| **Required tests** | C0-TEST-MAT-3 (basic water movement), additional: water at edge, water pooling behavior |

> **ASSUMPTION (conservative)**: Water cells are iterated bottom-up (high y first), left-to-right within each row. First water cell to claim a destination wins. **Matches falling-sand conventions.**

---

## 4. Moisture Diffusion

| Attribute | Value |
|---|---|
| **Inputs** | All cells in stable order |
| **Outputs** | Moisture transfer between cells (4 units if delta > 16) |
| **Deterministic order** | "Stable order" — assumed row-major (y=0..127, x=0..127 within each y). Double-buffer or stable pass required to avoid order-dependent cascading. |
| **Pseudocode section** | Spec `03` L130–156 |
| **Missing precision** | "Stable pass or double-buffer" — which one? This affects results. With in-place updates, moisture flows preferentially in the iteration direction. With double-buffer, diffusion is symmetric. Spec says "stable pass or double-buffer" without choosing. Transfer is 4 units — is this per neighbor pair? Can a cell lose more than 16 moisture in one tick (4 × 4 neighbors)? |
| **Edge cases** | Cell with moisture=255 next to cell with moisture=0 — delta=255 > 16, transfers 4. Very slow. Cell surrounded by 4 low-moisture neighbors — loses 16 per tick max. Water cell (source) — is it always 255? Does it ever lose moisture to diffusion? |
| **Required tests** | C0-TEST-MAT-3, C0-TEST-MAT-4 (moisture/scent interaction), additional: verify diffusion rate, verify symmetry |

> **Pre-v0.3.3 finding (historical)**: "Stable pass or double-buffer" — which one? ... **Flagged for Chaz review.**
> **v0.3.3 resolution**: Locked to explicit double-buffer (source buffer = start-of-step moisture values; destination buffer receives all writes; update order cannot affect results; copy/swap after full pass). "Stable pass" variant not used. See spec_pack/03 L136-151 ("Double-buffer contract (locked v0.3.3)"). **RESOLVED IN v0.3.3.**

---

## 5. Wet LooseSoil Support Loss

| Attribute | Value |
|---|---|
| **Inputs** | LooseSoil cells with moisture > 120 |
| **Outputs** | support = support.saturating_sub(1) per tick |
| **Deterministic order** | Part of moisture diffusion pass or separate pass (unspecified) |
| **Pseudocode section** | Spec `03` L158–163 |
| **Missing precision** | When in the tick order does this happen? It's listed under moisture diffusion (step 6) but could be a separate sub-step. Does it happen before or after moisture diffusion itself? |
| **Edge cases** | LooseSoil with support=0 and moisture>120 — already at minimum. LooseSoil that becomes wet this tick from diffusion — does it lose support this same tick? |
| **Required tests** | C0-TEST-WET-1 (wet support loss), C0-TEST-MAT-2 (collapse triggered by moisture-weakened support) |

---

## 6. Scent Decay

| Attribute | Value |
|---|---|
| **Inputs** | All cells |
| **Outputs** | scent_home -= 1; scent_food -= 1 (saturating). Water cells: both scents = 0. Wet cells (moisture > 100): extra -2 (saturating). |
| **Deterministic order** | Tick step 7: after moisture diffusion. Applied to all cells in stable order. |
| **Pseudocode section** | Spec `03` L167–188 |
| **Missing precision** | Order of operations: base decay first, then water zero, then wet extra? Or does water zeroing make wet-extra irrelevant for Water cells? (If Water zeros first, wet-extra on Water is moot.) Does wet-extra apply to the result after base decay or to the original value? |
| **Edge cases** | Cell with scent_home=1, moisture=150: base decay → 0, wet extra → already 0 (saturating). Cell that was just reinforced this tick (step 8 is after step 7) — reinforcement is applied after decay. |
| **Required tests** | C0-TEST-MAT-5 (scent decay), C0-TEST-MAT-4 (moisture/scent interaction) |

> **ASSUMPTION (conservative)**: Base decay first, then water zeroing, then wet extra. Order is: steps applied sequentially per cell. **Matches pseudocode order.**

---

## 7. Scent Reinforcement

| Attribute | Value |
|---|---|
| **Inputs** | Ant group position; task (ReturnHome → home scent, Forage+food → food scent); harvest events |
| **Outputs** | scent_home += 12 or scent_food += 12 (capped at 255); harvest → +24 food scent on current/adjacent cell |
| **Deterministic order** | Tick step 8: after scent decay |
| **Pseudocode section** | Spec `03` L192–205 |
| **Missing precision** | Harvest +24: "current cell or adjacent traversable cell" — which one? If current cell is not traversable (ant is adjacent to carcass, not on it), which adjacent cell gets the bonus? Deterministic selection needed. |
| **Edge cases** | Scent at 250 + 12 = 262 → capped at 255. Ant on Water cell — scent was zeroed in step 7, now reinforced in step 8 — net effect is +12 from 0, then zeroed again next tick. |
| **Required tests** | C0-TEST-MAT-6 (scent reinforcement) |

> **Pre-v0.3.3 finding (historical)**: Harvest +24: "current cell or adjacent traversable cell" — which one? ... **Flagged for Chaz review.**
> **v0.3.3 resolution**: Exact deterministic rule: If AntGroup current cell is traversable, apply +24 there; else first adjacent traversable in fixed order up, right, down, left; always clamp to [0,255]. See spec_pack/03 L210-216 ("Fixed deterministic rule (v0.3.3)"). **RESOLVED IN v0.3.3.**

---

## 8. SourbackBitter Residue Slowdown

| Attribute | Value |
|---|---|
| **Inputs** | Ant group position (on SourbackBitter cell); task = Scout or Forage; tick_index |
| **Outputs** | Movement allowed only on even tick_index; AntGroupSlowed event on odd tick |
| **Deterministic order** | Part of tick step 2 (ant group movement) |
| **Pseudocode section** | Spec `03` L217–222 |
| **Missing precision** | "Even tick_index" — is tick 0 even (yes, mathematically)? Does the check use the tick_index at the start of the tick or after increment? "If not already emitted for this command" — how is this tracked? Per-command flag on ant group? |
| **Edge cases** | tick_index = 0 (first tick) — even, so movement allowed. Ant enters residue on even tick, moves; next tick (odd), slowed — is the event emitted only once per command or once per odd tick? |
| **Required tests** | C0-TEST-ANT-1 (residue slowdown) |

> **ASSUMPTION (conservative)**: tick_index % 2 == 0 allows movement. Event emitted once per odd tick while in residue, not just once per command. **Spec wording "if not already emitted for this command" suggests once per command.** Contradicts "ant may move only on even tick_index" which implies continuous effect. **Flagged for Chaz review.**

---

## 9. Worker Loss (SourbackBitter)

| Attribute | Value |
|---|---|
| **Inputs** | Ant group; task = Forage; first entry into SourbackBitter during current command |
| **Outputs** | lost = min(3, workers); workers -= lost; confidence -= fixed penalty; WorkerLoss event |
| **Deterministic order** | Part of tick step 2 (ant group movement) |
| **Pseudocode section** | Spec `03` L226–232 |
| **Missing precision** | "First entry during that command" — how is "first entry" tracked? Is it per-command or per-tick? What is the "fixed penalty" for confidence? Not specified numerically. What if workers = 0 at time of entry? |
| **Edge cases** | workers = 2: lose min(3, 2) = 2. workers = 0: lose 0, no event? Entry = ant moves onto residue cell, or also being on residue when command starts? |
| **Required tests** | C0-TEST-ANT-2 (worker loss) |

> **Pre-v0.3.3 finding (historical)**: "fixed penalty" for confidence? Not specified numerically. ... **exact value not specified — flagged for Chaz.**
> **v0.3.3 resolution**: Fixed penalty = 16, clamped to [0, 255] (confidence = confidence.saturating_sub(16)). WorkerLoss event + perception receipt emitted. See spec_pack/03 L241 and spec_pack/04 L127. **RESOLVED IN v0.3.3.**

---

## 10. Carcass Harvesting

| Attribute | Value |
|---|---|
| **Inputs** | Ant group; task = Forage; adjacent Carcass cell |
| **Outputs** | One Carcass cell → Air; food_carried += 1; CarcassHarvested event |
| **Deterministic order** | Part of tick step 3 (digging/harvesting effects). Adjacent target order: up, right, down, left |
| **Pseudocode section** | Spec `03` L238–253 |
| **Missing precision** | "No harvesting through solid soil" — does this mean the Carcass cell must be accessible without digging, or that the ant must be adjacent on a traversable cell? What about diagonal adjacency? |
| **Edge cases** | Ant adjacent to multiple Carcass cells — deterministic order picks first valid. Last Carcass cell harvested — carcass completely gone. Ant on Water cell adjacent to Carcass — allowed? (Water is traversable but slow.) |
| **Required tests** | C0-TEST-MAT-7 (carcass harvest) |

> **ASSUMPTION (conservative)**: "Adjacent" means cardinal only (not diagonal), matching the up/right/down/left order. Ant must be on a traversable cell. "No harvesting through solid soil" means the Carcass cell itself must be directly cardinally adjacent to the ant's current traversable cell, with no solid cell between them (trivially satisfied by cardinal adjacency).

---

## 11. Command Failure

| Attribute | Value |
|---|---|
| **Inputs** | Any command with failed preconditions |
| **Outputs** | CommandFailed event with reason; chunk state unchanged (except receipt/event log) |
| **Deterministic order** | Tick step 1 (command intent application) or step 2 (execution) |
| **Pseudocode section** | Spec `04` L87–94 |
| **Missing precision** | Is the failure detected at command queuing (step 1) or execution (step 2)? Spec has both preconditions and failure events. Some failures can only be detected during execution (e.g., Blocked during movement). |
| **Edge cases** | Multiple commands queued (spec doesn't discuss command queuing beyond one at a time). Command that partially succeeds then fails. |
| **Required tests** | C0-TEST-CMD-1 (blocked dig) |

---

## 12. Chunk Boundary Behavior

| Attribute | Value |
|---|---|
| **Inputs** | Any simulation rule operating near x=0, x=127, y=0, y=127 |
| **Outputs** | No off-boundary effects; water stops; ants stop; collapse stops |
| **Deterministic order** | All rules must check boundaries |
| **Pseudocode section** | Spec `02` L28–32 |
| **Missing precision** | Are boundary cells themselves modifiable (e.g., can stone floor at y=124 be changed)? The spec says y=124..128 is stone — Stone is not diggable, so they can't change by normal rules. But what about moisture diffusion into stone? (Material table says Stone doesn't accept moisture — OK.) |
| **Edge cases** | Water at x=0 trying to flow left — blocked. Ant at y=0 trying to move up — blocked. Collapse check at y=126: below is y=127 which is stone — no collapse. Support reduction from digging at x=1 affects x=0 (stone boundary) — support changes on stone are meaningless but not harmful. |
| **Required tests** | C0-TEST-BOUND-1 (boundary behavior) |

> **Pre-v0.3.3 finding (historical)**: "treated as solid boundary behavior" ambiguous... **Missing precision** on whether boundary cells modifiable...
> **v0.3.3 resolution**: Boundary ring (x=0, x=127, y=0, y=127) are explicit immutable Stone cells in WorldTruth. Dig attempts blocked + CommandFailed(NotDiggable) receipted. Full ring is Stone. See spec_pack/02 L26-34 ("Boundary Behavior (v0.3.3 locked)"). **RESOLVED IN v0.3.3.**

---

## Rule Interaction Summary

| Tick Step | Rule | Reads | Writes | Can Trigger |
|---|---|---|---|---|
| 1 | Command intent | Command queue | Ant group intent | — |
| 2 | Ant movement/task | Chunk cells, ant state | Ant position/task/food | Steps 3, 8, 9 |
| 3 | Dig/harvest effects | Ant state, target cells | Cell material/support/flags | Steps 4 (collapse from digging), 9 |
| 4 | Gravity/collapse | All cells (bottom-up) | Cell swaps, flags | Step 9 |
| 5 | Water flow | Water cells (bottom-up) | Cell swaps, moisture | Step 6 |
| 6 | Moisture diffusion | All cells | Moisture values, LooseSoil support | Step 4 (next tick) |
| 7 | Scent decay | All cells | Scent values | — |
| 8 | Scent reinforcement | Ant position/task | Scent values | — |
| 9 | Perception events | All changes this tick | Event ledger | — |
| 10 | Receipt/render | All state | CommandReceipt, RenderFrame | — |

# 10 — Consistency Audit

Contradictions, vague terms, and risky ambiguity found across the v0.3.2 spec.

---

## Issues

### CA-001: Tunnel Support Contradiction Between v0.3.1 and v0.3.2

| Attribute | Value |
|---|---|
| **ID** | CA-001 |
| **Source files** | `spec_pack/02` L46 (v0.3.2), `source_review/spec_pack_02` L18 (v0.3.1) |
| **Conflicting wording** | v0.3.1: Nest/Tunnel initial support = 255. v0.3.2: Nest/Tunnel initial support = 0. |
| **Why it matters** | Support = 255 means the tunnel is structurally rock-solid. Support = 0 means it's structurally empty (which makes sense for open space). The correction is likely intentional—Tunnel is open space like Air—but not called out in the correction report. |
| **Proposed correction** | Confirm v0.3.2 value (support = 0) is intentional. Add a note in spec `02` or correction report explaining the change. |
| **Severity** | Major |

---

### CA-002: NestWall Traversability Correction Not Propagated

| Attribute | Value |
|---|---|
| **ID** | CA-002 |
| **Source files** | `spec_pack/01` L77 (v0.3.2 correction note), `spec_pack/04` L54–63 (v0.3.2 movement table) |
| **Conflicting wording** | Spec `01` L77 says "Correction: NestWall is not ant-traversable." Spec `04` L56–63 lists NestWall in the "may not move through" list. However, v0.3.1 spec_01 L24 listed NestWall as traversable. The correction is applied but the inline correction note in `01` L77 is unusual and may confuse implementers. |
| **Why it matters** | If an agent reads the material semantics table without the inline correction, they might miss that NestWall changed from traversable to non-traversable. |
| **Proposed correction** | Remove the inline "Correction:" note from `01` L77. Instead, change the material semantics table row for NestWall to show `No` under "Traversable by AntGroup?" directly. The correction is already in the table (L75 shows "No"). The note is redundant. |
| **Severity** | Minor |

---

### CA-003: `board_deltas` / `result_card` — Residual Risk

| Attribute | Value |
|---|---|
| **ID** | CA-003 |
| **Source files** | `SOURCE_REVIEW_NOTES.md` L11, `SPEC_V0_3_2_CORRECTION_REPORT.md` L28–29, `skills/SKILL.md` L69–75 |
| **Conflicting wording** | The correction report and skill file document that `board_deltas` and `result_card` were replaced. However, prior to v0.3.3 neither performed the exact required assertion. |
| **Why it matters** | If any `board_deltas` or `result_card` text survived in the spec files, an implementer might use the wrong terminology. |
| **Proposed correction** | Grep confirmed (v0.3.3): no normative uses in v0.3.2/v0.3.3 spec pack; terms appear only in the banned/avoid terminology list. (Banned blocks preserved in spec_pack/05 and skills/SKILL.md.) No further action needed. |
| **Severity** | Editorial (verified clean) |

---

### CA-004: `pixels` / `wgpu` Contradiction — Clarification Present but Easy to Miss

| Attribute | Value |
|---|---|
| **ID** | CA-004 |
| **Source files** | `spec_pack/06` L153–165 |
| **Conflicting wording** | L155 lists "GPU-owned simulation" and L156 lists "custom GPU compute" as banned. L165 clarifies: "`pixels` may use `wgpu` internally for rendering pixels to the screen. The ban is on custom GPU compute and GPU-owned simulation, not on transitive rendering internals." |
| **Why it matters** | An implementer seeing "GPU" in the banned list might reject `pixels` as a fallback. The clarification at L165 resolves this, but it's at the bottom of the dependency section, easy to miss. |
| **Proposed correction** | Move the wgpu clarification immediately after the banned list, or add a footnote marker on the GPU entries pointing to the clarification. |
| **Severity** | Minor |

---

### CA-005: "Implementation Ready" Wording Ambiguity

| Attribute | Value |
|---|---|
| **ID** | CA-005 |
| **Source files** | `spec_pack/09` title: "Implementation Handoff After Approval" |
| **Conflicting wording** | The file title says "After Approval" and L5 says "Use this file only after Chaz approves." However, the file exists in the workspace now, visible to agents. An agent might interpret the existence of module sketches and API contracts as authorization to begin implementation. |
| **Why it matters** | This is the spec's primary defense against premature implementation. The file must be maximally clear. |
| **Proposed correction** | Add a large warning banner at the top of spec `09`: `> ⚠️ IMPLEMENTATION NOT AUTHORIZED. This file is a post-approval reference only.` The current "Use this file only after Chaz approves" is correct but could be more prominent. |
| **Severity** | Major |

---

### CA-006: Coordinate Range Boundary Inconsistency

| Attribute | Value |
|---|---|
| **ID** | CA-006 |
| **Source files** | `spec_pack/02` L8–9, L28–29 |
| **Conflicting wording** | L8–9: "x: 0..128, y: 0..128" (half-open, so valid indices are 0–127). L28: "x=0, x=127, y=0, and y=127 are treated as solid boundary behavior." — This means the usable interior is 1..127 (126×126). But the layout table at L40–49 uses the full 0..128 range for Air, Stone, etc. |
| **Why it matters** | "Solid boundary behavior" at x=0 and y=0 means the outermost ring of cells has special treatment, but the layout table initializes those cells normally (e.g., Air at y=0..36 includes y=0). If y=0 is "solid boundary," is it Air or Stone? The boundary behavior seems to mean "nothing flows off-screen," not "physical boundary cells are different material." |
| **Proposed correction** | Clarify that "solid boundary behavior" means simulation rules treat edges as impassable for flow/movement, but the material in those cells is defined by the layout table. This is likely the intended meaning. Alternatively, if boundary cells should be Stone, the layout table needs updating. |
| **Severity** | Major |

---

### CA-007: Moisture Diffusion — Single-Buffer vs Double-Buffer Unresolved

| Attribute | Value |
|---|---|
| **ID** | CA-007 |
| **Source files** | `spec_pack/03` L131 |
| **Conflicting wording** | "Moisture diffusion is deterministic and conservative enough for Chunk 0." L131 uses phrase "stable pass or double-buffer" — this is an OR, not a locked decision. |
| **Why it matters** | Single-buffer and double-buffer produce different diffusion patterns. Both are deterministic, but different. An implementer must choose one, which affects the golden hash. |
| **Proposed correction** | Lock this to one approach. Recommendation: single-buffer (in-place) for simplicity, matching the falling-sand convention used for other rules. Or double-buffer for symmetric diffusion. **Flagged for D25 decision.** |
| **Severity** | Major |
| **Previous finding (pre-v0.3.3)** | "stable pass or double-buffer" — this is an OR, not a locked decision. **Flagged for D25 decision.** |
| **v0.3.3 resolution** | Locked to explicit double-buffer (source buffer = start-of-step values; destination buffer receives writes; update order cannot affect results; swap/copy after full pass). See spec_pack/03 L136-151 ("Double-buffer contract (locked v0.3.3)"). **RESOLVED IN v0.3.3.** |

---

### CA-008: Confidence Penalty Value Not Specified

| Attribute | Value |
|---|---|
| **ID** | CA-008 |
| **Source files** | `spec_pack/03` L229, `spec_pack/04` L142–155 |
| **Conflicting wording** | Spec `03` L229: "confidence -= fixed penalty." Spec `04` L142–148 lists what decreases confidence but gives no numeric values. |
| **Why it matters** | "Fixed penalty" is not a number. An implementer must invent one, which could affect gameplay balance and determinism (if it changes across implementations). |
| **Proposed correction** | Specify exact confidence penalty values. E.g., worker loss: -20; bitter residue observed: -10; route blocked: -15. Or mark confidence values as explicitly implementation-flexible with test bounds. |
| **Severity** | Major |
| **Previous finding (pre-v0.3.3)** | "confidence -= fixed penalty." ... no numeric values. **Major** (implementer must invent one). |
| **v0.3.3 resolution** | Fixed penalty = 16, clamped to [0, 255] (confidence = confidence.saturating_sub(16)). WorkerLoss + receipt/perception aligned. See spec_pack/03 L241 and spec_pack/04 L127. **RESOLVED IN v0.3.3.** |

---

### CA-009: Scent Reinforcement Harvest Location Ambiguity

| Attribute | Value |
|---|---|
| **ID** | CA-009 |
| **Source files** | `spec_pack/03` L203–205 |
| **Conflicting wording** | "current cell or adjacent traversable cell gains scent_food +24" — which one? If ant is adjacent to carcass (on a traversable cell), the "current cell" is the ant's position. But "or adjacent traversable cell" introduces ambiguity. |
| **Why it matters** | Determinism requires exactly one answer. If two implementations choose different cells, hashes diverge. |
| **Proposed correction** | Change to: "The ant group's current cell gains scent_food +24 on successful harvest." Remove "or adjacent traversable cell." |
| **Severity** | Major |
| **Previous finding (pre-v0.3.3)** | "current cell or adjacent traversable cell gains scent_food +24" — ambiguity. Old proposal favored "current cell only". **Major**. |
| **v0.3.3 resolution** | Deterministic: If AntGroup current cell traversable → +24 there; else first adjacent traversable in fixed order (up, right, down, left); clamp [0,255]. See spec_pack/03 L210-216 ("Fixed deterministic rule (v0.3.3)"). **RESOLVED IN v0.3.3.** |

---

### CA-010: AntGroupSlowed Event Emission Frequency

| Attribute | Value |
|---|---|
| **ID** | CA-010 |
| **Source files** | `spec_pack/03` L221 |
| **Conflicting wording** | "odd tick emits AntGroupSlowed if not already emitted for this command" — suggests once per command. But the movement restriction (even ticks only) is continuous while in residue. Does the event fire once per command or once per odd tick? |
| **Why it matters** | Event frequency affects the perception ledger size and test expectations. |
| **Proposed correction** | Clarify: "AntGroupSlowed is emitted once per command, on the first odd tick where movement is skipped due to SourbackBitter residue." |
| **Severity** | Minor |

---

### CA-011: Collapse and Ant Group Interaction Unspecified

| Attribute | Value |
|---|---|
| **ID** | CA-011 |
| **Source files** | `spec_pack/03` L94–108 |
| **Conflicting wording** | Collapse pseudocode swaps LooseSoil with the cell below. But what if the ant group is on the cell below? Spec is silent. |
| **Why it matters** | Ant group could be buried or crushed by collapse. Ignoring this leads to undefined behavior. |
| **Proposed correction** | Add a rule: "If the ant group occupies the destination cell of a collapse swap, the collapse is blocked (the cell does not fall into an occupied space)." Or: "The ant group is pushed to the nearest traversable cell." **Flagged for Chaz review.** |
| **Severity** | Major |
| **Previous finding (pre-v0.3.3)** | Spec silent on AntGroup in destination cell. Proposals: blocked or pushed. **Flagged for Chaz review.** **Major**. |
| **v0.3.3 resolution** | Material swap still occurs (world-truth deterministic). AntGroup in dest cell receives *existing* worker-loss consequence (min(3,workers), confidence -=16 clamped), WorkerLoss event + receipt/perception. No health/animation/combat invented. See spec_pack/03 L110-116. **RESOLVED IN v0.3.3.** |

---

### CA-012: Water Flow Iteration Order Within Row Not Specified

| Attribute | Value |
|---|---|
| **ID** | CA-012 |
| **Source files** | `spec_pack/03` L114 |
| **Conflicting wording** | "For each water cell in bottom-up order" — specifies y-order (bottom-up) but not x-order within a row. Left-to-right is conventional but not stated. |
| **Why it matters** | X-order affects which water cell claims a shared destination first. Different x-order = different water behavior = different hash. |
| **Proposed correction** | Add: "Within each row, water cells are processed left-to-right (x=0 to x=127)." |
| **Severity** | Minor |

---

### CA-013: Missing `FIXTURE_BLOCKED_DIG` Layout

| Attribute | Value |
|---|---|
| **ID** | CA-013 |
| **Source files** | `spec_pack/07` L12 |
| **Conflicting wording** | FIXTURE_BLOCKED_DIG is listed in the fixture table but no layout definition is provided (unlike FIXTURE_INITIAL which has spec `02`). |
| **Why it matters** | Implementer must invent the fixture layout, which may not match spec intent. |
| **Proposed correction** | Add a brief layout description for all non-INITIAL fixtures in spec `07` or a new spec file. At minimum: stone/root wall adjacent to ant, with dig target behind it. |
| **Severity** | Minor |

---

### CA-014: Digging Neighbor Support Reduction — Cardinal Order

| Attribute | Value |
|---|---|
| **ID** | CA-014 |
| **Source files** | `spec_pack/03` L85–86 |
| **Conflicting wording** | "for each cardinal neighbor: neighbor.support = saturating_sub(neighbor.support, 50)" — order of processing cardinal neighbors is not specified. |
| **Why it matters** | Cardinal neighbor processing order doesn't affect results here (each neighbor independently loses 50), so this is truly order-independent. However, the inconsistency with the harvest rule (which specifies up/right/down/left) might confuse implementers. |
| **Proposed correction** | Specify cardinal neighbor order for consistency (up, right, down, left — matching harvest). Not functionally necessary but improves spec consistency. |
| **Severity** | Editorial |

---

### CA-015: Hidden Return to Card-Dashboard Thinking

| Attribute | Value |
|---|---|
| **ID** | CA-015 |
| **Source files** | `spec_pack/05` L121–141 |
| **Conflicting wording** | The example CommandReceipt JSON at L123–138 is a rich JSON object with nested structures. This is correct for internal data. But the `dev_event_summary` field (L136: "Digging opened one cell. The residue remains unexplained.") reads like a card narrative. |
| **Why it matters** | If `dev_event_summary` becomes a polished narrative sentence, agents may focus on writing good narratives instead of building the grid. The summary is a debug tool, not prose. |
| **Proposed correction** | Clarify that `dev_event_summary` is a terse debug string, not a narrative. Example: "dig:1_cell, residue:unexplained" rather than prose. Or explicitly mark the example as illustrative, not normative format. |
| **Severity** | Minor |

---

### CA-016: v0.3.1 Claimed "All Decisions Resolved" Prematurely

| Attribute | Value |
|---|---|
| **ID** | CA-016 |
| **Source files** | `source_review/spec_pack_12` L1–2 |
| **Conflicting wording** | v0.3.1 spec_12 says "All critical decisions for Chunk 0 are now resolved." v0.3.2 correctly downgrades to "contract candidates, not yet approved." |
| **Why it matters** | Already corrected in v0.3.2. Historical record only. But agents reading v0.3.1 source files should not treat decisions as approved. |
| **Proposed correction** | No action needed on v0.3.2 files. The v0.3.1 files are historical. Consider adding a README to `source_review/` folder stating these are superseded. |
| **Severity** | Editorial |

---

### CA-017: Scent Diffusion Missing from Spec

| Attribute | Value |
|---|---|
| **ID** | CA-017 |
| **Source files** | `spec_pack/03` L167–205 |
| **Conflicting wording** | Spec has scent decay and scent reinforcement but no scent diffusion (lateral spreading of scent to neighbor cells). Research indicates diffusion is important for ants to follow trails. |
| **Why it matters** | Without diffusion, scent trails are exactly 1 cell wide. The ant group uses greedy movement, not gradient sensing, so this may be acceptable for Chunk 0. But it could make scent overlays look like single-pixel dots rather than trails. |
| **Proposed correction** | Either: (a) add scent diffusion to spec `03` as a rule similar to moisture diffusion, or (b) explicitly note that scent diffusion is deferred to Chunk 2. Option (b) is safer for Chunk 0 scope. |
| **Severity** | Minor |

---

### CA-018: `ColonyPerception` vs `ColonyView` Naming

| Attribute | Value |
|---|---|
| **ID** | CA-018 |
| **Source files** | `spec_pack/04` L103 uses `ColonyPerception`; `spec_pack/05` L7 uses `ColonyPerception`; `spec_pack/06` L52/75/79 uses `ColonyView`; `skills/SKILL.md` L28/64 uses `ColonyView` |
| **Conflicting wording** | The same concept has two names: `ColonyPerception` (data model) and `ColonyView` (rendering mode). They are related but not identical — ColonyPerception is the data, ColonyView is the presentation of that data. |
| **Why it matters** | Implementers may confuse the data model with the rendering mode, or create redundant structs. |
| **Proposed correction** | Add a glossary note: "ColonyPerception = the epistemic data model (what the colony knows). ColonyView = the rendering mode that presents ColonyPerception to the player." |
| **Severity** | Minor |

---

## Issue Summary

| Severity | Count | IDs |
|---|---|---|
| **Major** | 6 | CA-001, CA-006, CA-007, CA-008, CA-009, CA-011 |
| **Minor** | 6 | CA-002, CA-004, CA-010, CA-012, CA-015, CA-017, CA-018 |
| **Editorial** | 3 | CA-003, CA-014, CA-016 |
| **Blocker** | 0 | — |
| **Total** | **18** | — |

> **Key finding**: No blockers, but 6 major issues require resolution before implementation. The most critical are CA-007 (moisture buffer choice), CA-008 (confidence penalty values), CA-009 (harvest scent location), and CA-011 (collapse + ant group interaction).

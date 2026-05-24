# Source Review Notes — Gemini v0.3.1 DOCX Ingest (Historical Provenance Only)

**WARNING (v0.3.3):** source_review/ is historical provenance ONLY. It contains v0.3.1 Gemini DOCX-derived .txt files from the initial ingest. It must NOT be used as normative implementation input or current spec canon. The normative candidate awaiting Chaz approval is spec_pack/ (v0.3.2 + v0.3.3 targeted corrections) plus accepted patches from agent_mapping_pass_01/. Agents and implementers must treat source_review/ files as superseded archival record only.

## Reviewed Input

Uploaded archive: `geminispecs-20260524T015938Z-3-001.zip`

The archive contained DOCX files with Markdown-like names. The files were extracted and reviewed for consistency with the current TWIOFA Chunk 0 direction.

Grep confirmed: no normative uses in v0.3.2/v0.3.3 spec pack; board_deltas and result_card terms appear only in the banned/avoid terminology list.

## Corrections Required

1. **Output terminology drift:** Grep confirmed: no normative uses in v0.3.2/v0.3.3 spec pack; board_deltas and result_card terms appear only in the banned/avoid terminology list. (Banned blocks left intact in spec_pack/05 and skills/SKILL.md.)
2. **Visualizer truth boundary:** Gemini's version allowed a canvas to receive chunk data, but the player-facing mode must not receive hidden semantic truth. Added `RenderFrame` and `VisibleCell` boundaries.
3. **WorldTruth oversimplification:** `WorldTruth` was sometimes reduced to `Vec<Cell>`. Corrected: `Vec<Cell>` is only the main substrate.
4. **Cell-size claim:** Prior 6-byte/8-byte alignment language was unsafe. Corrected: exact size must be verified by `size_of::<Cell>()` if required.
5. **Carcass mass contradiction:** The cell schema had no mass field, while harvest said “after N actions.” Corrected: one carcass cell equals one harvestable unit in Chunk 0.
6. **Residue slowdown ambiguity:** “50% movement skip” was made deterministic via tick parity / cooldown rules.
7. **Validation gaps:** Traceability and risk-register files were effectively empty. Filled out with actionable matrices.
8. **Implementation gating:** Reasserted that no coding is authorized until Chaz approves the v0.3.2 build-readiness gate.

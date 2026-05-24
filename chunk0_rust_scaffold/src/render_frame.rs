use crate::cell::{Flags, Material, Residue};
use crate::chunk::{Chunk, HEIGHT, WIDTH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibleCell {
    pub coord: (u8, u8),
    pub visible_material_category: String,
    pub visible_color_id: u8,
    pub visible_overlay_value: u8,
    pub known_perception_marker: Option<String>,
    pub recent_delta_marker: Option<String>,

    // Developer debug fields (only populated in DevTruth mode)
    pub dev_material: Option<Material>,
    pub dev_moisture: Option<u8>,
    pub dev_scent_home: Option<u8>,
    pub dev_scent_food: Option<u8>,
    pub dev_residue: Option<Residue>,
    pub dev_support: Option<u8>,
    pub dev_flags: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderFrame {
    pub tick_index: u32,
    pub chunk_hash: String,
    pub visible_cells: Vec<VisibleCell>,
    pub overlay_mode: String,
    pub chunk_deltas: Vec<crate::orders::CellDelta>,
    pub perception_markers: Vec<String>,
    pub debug_stats: crate::orders::DebugStats,
}

impl RenderFrame {
    pub fn generate(
        tick_index: u32,
        chunk_hash: String,
        chunk: &Chunk,
        overlay_mode: &str,
        chunk_deltas: Vec<crate::orders::CellDelta>,
        perception_markers: Vec<String>,
        debug_stats: crate::orders::DebugStats,
        sourback_earned: bool,
        dev_mode: bool,
    ) -> Self {
        let mut visible_cells = Vec::with_capacity(chunk.cells.len());

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = chunk.get(x, y);
                let coord = (x as u8, y as u8);

                // Material category mapping
                let visible_material_category = format!("{:?}", cell.material);
                let visible_color_id = cell.material as u8;

                // Scent/Moisture overlays mapping
                let visible_overlay_value = match overlay_mode {
                    "Moisture" => cell.moisture,
                    "ScentHome" => cell.scent_home,
                    "ScentFood" => cell.scent_food,
                    _ => 0,
                };

                // Guardrail residue to ColonyView descriptions
                let known_perception_marker = match cell.residue {
                    Residue::SourbackBitter => {
                        if sourback_earned {
                            Some("Sourback-associated path".to_string())
                        } else {
                            Some("bitter/yellow residue".to_string())
                        }
                    }
                    Residue::None => None,
                    _ => Some("unfamiliar residue".to_string()),
                };

                let recent_delta_marker = if (cell.flags & Flags::RECENTLY_DUG) != 0 {
                    Some("recently dug".to_string())
                } else if (cell.flags & Flags::RECENTLY_COLLAPSED) != 0 {
                    Some("recently collapsed".to_string())
                } else {
                    None
                };

                let mut visible_cell = VisibleCell {
                    coord,
                    visible_material_category,
                    visible_color_id,
                    visible_overlay_value,
                    known_perception_marker,
                    recent_delta_marker,
                    dev_material: None,
                    dev_moisture: None,
                    dev_scent_home: None,
                    dev_scent_food: None,
                    dev_residue: None,
                    dev_support: None,
                    dev_flags: None,
                };

                if dev_mode {
                    // Expose raw truth for debugging only
                    visible_cell.dev_material = Some(cell.material);
                    visible_cell.dev_moisture = Some(cell.moisture);
                    visible_cell.dev_scent_home = Some(cell.scent_home);
                    visible_cell.dev_scent_food = Some(cell.scent_food);
                    visible_cell.dev_residue = Some(cell.residue);
                    visible_cell.dev_support = Some(cell.support);
                    visible_cell.dev_flags = Some(cell.flags);
                }

                visible_cells.push(visible_cell);
            }
        }

        Self {
            tick_index,
            chunk_hash,
            visible_cells,
            overlay_mode: overlay_mode.to_string(),
            chunk_deltas,
            perception_markers,
            debug_stats,
        }
    }
}

use crate::events::Event;

/// Translates objective events into subjective, colony-safe natural language descriptions,
/// strictly enforcing the epistemic boundary (e.g. guardrailing "Sourback" mentions).
pub fn translate_event_to_colony(event: &Event, sourback_earned: bool) -> String {
    match event {
        Event::CellDug { x, y, from, to } => {
            format!(
                "Colony opened tunnel at ({}, {}), clearing {:?} into {:?}",
                x, y, from, to
            )
        }
        Event::WorkerLoss { lost, x, y, reason } => {
            let safe_reason = if reason.contains("Sourback") || reason.contains("bitter") {
                if sourback_earned {
                    "Sourback-associated hazard"
                } else {
                    "bitter/yellow residue"
                }
            } else {
                reason.as_str()
            };
            format!(
                "Lost {} workers at ({}, {}) due to {}",
                lost, x, y, safe_reason
            )
        }
        Event::CommandFailed { reason, command } => {
            format!("Command '{}' failed: {}", command, reason)
        }
        Event::CollapseOccurred { x, y } => {
            format!("Tunnel collapsed at ({}, {})", x, y)
        }
        Event::CarcassHarvested { x, y, food_units } => {
            format!("Harvested {} food units at ({}, {})", food_units, x, y)
        }
        Event::AntGroupSlowed { x, y, reason } => {
            let safe_reason = if reason.contains("Sourback") || reason.contains("bitter") {
                if sourback_earned {
                    "Sourback-associated path resistance"
                } else {
                    "bitter/yellow residue path resistance"
                }
            } else {
                reason.as_str()
            };
            format!(
                "Ant movement slowed at ({}, {}) due to {}",
                x, y, safe_reason
            )
        }
        Event::FoodDeposited { amount } => {
            format!("Colony deposited {} food unit(s) at home", amount)
        }
    }
}

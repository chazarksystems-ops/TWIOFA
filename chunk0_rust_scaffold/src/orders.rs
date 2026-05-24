use crate::cell::Material;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    StepSimulation { ticks: u32 },
    DigTunnel { target: (u8, u8) },
    SendForagers { target: (u8, u8) },
    ScoutResidue { target: (u8, u8) },
    ReturnHome,
    Avoid { target: (u8, u8) },
    InspectCell { x: u8, y: u8 },
    Reset { seed: Option<u64> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellDelta {
    pub x: u8,
    pub y: u8,
    pub from: Material,
    pub to: Material,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugStats {
    pub ticks_advanced: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandReceipt {
    pub command_id: u64,
    pub tick_start: u32,
    pub tick_end: u32,
    pub chunk_hash_before: String,
    pub chunk_hash_after: String,
    pub chunk_deltas: Vec<CellDelta>,
    pub perception_updates: Vec<String>,
    pub dev_event_summary: String,
    pub debug_stats: DebugStats,
}

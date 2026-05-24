use crate::cell::Material;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    CellDug {
        x: u8,
        y: u8,
        from: Material,
        to: Material,
    },
    WorkerLoss {
        lost: u32,
        x: u8,
        y: u8,
        reason: String,
    },
    CommandFailed {
        reason: String,
        command: String,
    },
    CollapseOccurred {
        x: u8,
        y: u8,
    },
    CarcassHarvested {
        x: u8,
        y: u8,
        food_units: u32,
    },
    AntGroupSlowed {
        x: u8,
        y: u8,
        reason: String,
    },
    FoodDeposited {
        amount: u32,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PerceptionEventLedger {
    pub events: Vec<Event>,
}

impl PerceptionEventLedger {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

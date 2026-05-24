/// Fixed home/nest coordinate used by the ReturnHome task in Chunk 0.
/// Matches the AntGroup's default spawn position inside the nest band.
pub const HOME_COORD: (u8, u8) = (55, 118);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Task {
    Idle,
    Scout { target: (u8, u8) },
    Dig { target: (u8, u8) },
    Forage { target: (u8, u8) },
    ReturnHome,
    Avoid { target: (u8, u8) },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AntGroup {
    pub pos: (u8, u8),
    pub workers: u32,
    pub task: Task,
    pub food_carried: u32,
    pub fatigue: u8,
    pub confidence: u8,
}

impl Default for AntGroup {
    fn default() -> Self {
        Self {
            pos: (55, 118),
            workers: 100,
            task: Task::Idle,
            food_carried: 0,
            fatigue: 0,
            confidence: 255,
        }
    }
}

impl AntGroup {
    /// Appends the AntGroup's canonical byte representation to the provided buffer.
    pub fn write_canonical_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(self.pos.0);
        buf.push(self.pos.1);
        buf.extend_from_slice(&self.workers.to_be_bytes());
        match self.task {
            Task::Idle => {
                buf.push(0);
                buf.push(0);
                buf.push(0);
            }
            Task::Scout { target } => {
                buf.push(1);
                buf.push(target.0);
                buf.push(target.1);
            }
            Task::Dig { target } => {
                buf.push(2);
                buf.push(target.0);
                buf.push(target.1);
            }
            Task::Forage { target } => {
                buf.push(3);
                buf.push(target.0);
                buf.push(target.1);
            }
            Task::ReturnHome => {
                buf.push(4);
                buf.push(0);
                buf.push(0);
            }
            Task::Avoid { target } => {
                buf.push(5);
                buf.push(target.0);
                buf.push(target.1);
            }
        }
        buf.extend_from_slice(&self.food_carried.to_be_bytes());
        buf.push(self.fatigue);
        buf.push(self.confidence);
    }
}

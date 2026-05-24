#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Material {
    Air = 0,
    Soil = 1,
    LooseSoil = 2,
    Tunnel = 3,
    Water = 4,
    Carcass = 5,
    Root = 6,
    Stone = 7,
    NestWall = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Residue {
    None = 0,
    SourbackBitter = 1,
    Rot = 2,
    Alarm = 3,
}

pub struct Flags;
impl Flags {
    pub const RECENTLY_DUG: u8 = 1 << 0;
    pub const RECENTLY_COLLAPSED: u8 = 1 << 1;
    pub const OBSERVED_THIS_TICK: u8 = 1 << 2;
    pub const HARVESTED: u8 = 1 << 3;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub material: Material,
    pub moisture: u8,
    pub scent_home: u8,
    pub scent_food: u8,
    pub residue: Residue,
    pub support: u8,
    pub flags: u8,
}

impl Cell {
    pub fn new(material: Material, support: u8) -> Self {
        Self {
            material,
            moisture: 0,
            scent_home: 0,
            scent_food: 0,
            residue: Residue::None,
            support,
            flags: 0,
        }
    }

    /// Appends the cell's 7 fields to the provided byte buffer in a stable canonical layout.
    pub fn write_canonical_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(self.material as u8);
        buf.push(self.moisture);
        buf.push(self.scent_home);
        buf.push(self.scent_food);
        buf.push(self.residue as u8);
        buf.push(self.support);
        buf.push(self.flags);
    }
}

use crate::cell::{Cell, Material, Residue};

pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 128;
pub const CELL_COUNT: usize = WIDTH * HEIGHT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub cells: Vec<Cell>,
}

impl Chunk {
    pub fn index(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn in_bounds(x: usize, y: usize) -> bool {
        x < WIDTH && y < HEIGHT
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[Self::index(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[Self::index(x, y)]
    }
}

impl Default for Chunk {
    fn default() -> Self {
        // Initialize default Soil cells (support 200)
        let cells = vec![Cell::new(Material::Soil, 200); CELL_COUNT];
        let mut chunk = Self { cells };

        // 1. Air / surface
        for y in 0..36 {
            for x in 0..128 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::Air;
                cell.support = 0;
            }
        }

        // 2. Carcass edge
        for y in 18..28 {
            for x in 90..111 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::Carcass;
                cell.support = 100;
            }
        }

        // 3. Sourback residue path
        for y in 28..36 {
            for x in 80..116 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::Air;
                cell.residue = Residue::SourbackBitter;
                cell.support = 0;
            }
        }

        // 4. Root column
        for y in 36..116 {
            for x in 30..36 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::Root;
                cell.support = 255;
            }
        }

        // 5. Water pocket
        for y in 64..73 {
            for x in 60..71 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::Water;
                cell.moisture = 255;
                cell.support = 0;
            }
        }

        // 6. Loose soil risk band
        for y in 80..86 {
            for x in 50..81 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::LooseSoil;
                cell.support = 60;
            }
        }

        // 7. Nest / initial tunnel
        for y in 96..121 {
            for x in 45..66 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::Tunnel;
                cell.support = 0;
            }
        }

        // 8. Stone floor
        for y in 124..128 {
            for x in 0..128 {
                let cell = chunk.get_mut(x, y);
                cell.material = Material::Stone;
                cell.support = 255;
            }
        }

        // Overrides with immutable Stone boundary ring:
        // x = 0, x = 127, y = 0, y = 127
        for x in 0..128 {
            // y = 0
            let cell = chunk.get_mut(x, 0);
            *cell = Cell::new(Material::Stone, 255);

            // y = 127
            let cell = chunk.get_mut(x, 127);
            *cell = Cell::new(Material::Stone, 255);
        }

        for y in 0..128 {
            // x = 0
            let cell = chunk.get_mut(0, y);
            *cell = Cell::new(Material::Stone, 255);

            // x = 127
            let cell = chunk.get_mut(127, y);
            *cell = Cell::new(Material::Stone, 255);
        }

        chunk
    }
}

use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub control: f32,  // -1.0 to 1.0
    pub pressure: f32, // influence
}

#[derive(Resource)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![
            Cell {
                control: 0.0,
                pressure: 0.0,
            };
            width * height
        ];

        Self {
            width,
            height,
            cells,
        }
    }

    #[inline]
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[self.get_index(x, y)]
    }

    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let idx = self.get_index(x, y);
        &mut self.cells[idx]
    }
}

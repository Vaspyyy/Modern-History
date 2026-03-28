use crate::map::grid::Grid;
use bevy::prelude::*;

pub fn update_control(mut grid: ResMut<Grid>) {
    for cell in &mut grid.cells {
        cell.control += cell.pressure * 0.0001;
        cell.control = cell.control.clamp(-1.0, 1.0);
    }
}

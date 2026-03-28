use crate::core::GameConfig;
use crate::map::grid::Grid;
use bevy::prelude::*;

pub fn update_control(mut grid: ResMut<Grid>, config: Res<GameConfig>) {
    for cell in &mut grid.cells {
        cell.control += cell.pressure * config.control_speed;
        cell.control = cell.control.clamp(-1.0, 1.0);
    }
}

use crate::army::Army;
use crate::map::grid::Grid;
use bevy::prelude::*;

pub fn apply_pressure(mut grid: ResMut<Grid>, query: Query<&Army>) {
    let cell_size = 3.0;
    let influence_radius_cells = 10;

    for cell in &mut grid.cells {
        cell.pressure = 0.0;
    }

    for army in &query {
        let army_grid_x =
            ((army.position.x + grid.width as f32 * cell_size / 2.0) / cell_size) as isize;
        let army_grid_y =
            ((army.position.y + grid.height as f32 * cell_size / 2.0) / cell_size) as isize;

        for dy in -influence_radius_cells..=influence_radius_cells {
            for dx in -influence_radius_cells..=influence_radius_cells {
                let grid_x = army_grid_x + dx;
                let grid_y = army_grid_y + dy;

                if grid_x < 0
                    || grid_x >= grid.width as isize
                    || grid_y < 0
                    || grid_y >= grid.height as isize
                {
                    continue;
                }

                let distance_sq = (dx * dx + dy * dy) as f32;
                let _distance = distance_sq.sqrt();

                let influence = army.strength / (distance_sq + 100.0) * army.faction as f32;

                let cell = grid.get_mut(grid_x as usize, grid_y as usize);
                cell.pressure += influence;
            }
        }
    }
}

pub mod grid;

use bevy::prelude::*;
use grid::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(256, 256));
    }
}

pub fn setup_grid(mut grid: ResMut<Grid>) {
    let half_width = grid.width / 2;

    let border_width = 10usize;
    let left_edge = half_width.saturating_sub(border_width);
    let right_edge = (half_width + border_width).min(grid.width);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get_mut(x, y);

            if x < left_edge {
                cell.control = -1.0;
            } else if x >= right_edge {
                cell.control = 1.0;
            } else {
                let t = (x - left_edge) as f32 / (border_width * 2) as f32;
                cell.control = t * 2.0 - 1.0;
            }
        }
    }

    let left_value = grid.get(0, 0).control;
    let right_value = grid.get(grid.width - 1, 0).control;

    println!("Sample values: left={}, right={}", left_value, right_value);
}

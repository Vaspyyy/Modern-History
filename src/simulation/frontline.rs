use crate::map::grid::Grid;
use bevy::prelude::*;

pub fn detect_frontline(grid: &Grid, cell_size: f32) -> Vec<Vec2> {
    let mut frontline = Vec::new();
    let half_w = grid.width as f32 * cell_size / 2.0;
    let half_h = grid.height as f32 * cell_size / 2.0;
    let neighbors: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);
            if cell.control.abs() < 0.001 {
                continue;
            }

            for (dx, dy) in neighbors {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || nx >= grid.width as isize || ny < 0 || ny >= grid.height as isize {
                    continue;
                }

                let neighbor = grid.get(nx as usize, ny as usize);
                if cell.control * neighbor.control <= 0.0 {
                    let world_x = (x as f32 + dx as f32 * 0.5) * cell_size - half_w;
                    let world_y = (y as f32 + dy as f32 * 0.5) * cell_size - half_h;
                    frontline.push(Vec2::new(world_x, world_y));
                    break;
                }
            }
        }
    }

    frontline
}

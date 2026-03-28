use crate::map::grid::Grid;
use bevy::prelude::*;

#[derive(Component)]
pub struct CellVisual {
    pub x: usize,
    pub y: usize,
}

pub fn spawn_grid_visuals(mut commands: Commands, grid: Res<Grid>) {
    let cell_size = 3.0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);

            let color = if cell.control > 0.0 {
                Color::rgb(0.2, 0.4, 1.0)
            } else {
                Color::rgb(1.0, 0.2, 0.2)
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(cell_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x as f32 * cell_size - (grid.width as f32 * cell_size / 2.0),
                        y as f32 * cell_size - (grid.height as f32 * cell_size / 2.0),
                        0.0,
                    ),
                    ..default()
                },
                CellVisual { x, y },
            ));
        }
    }

    println!("Grid visuals spawned");
}

pub fn update_grid_visuals(grid: Res<Grid>, mut query: Query<(&CellVisual, &mut Sprite)>) {
    for (visual, mut sprite) in &mut query {
        let cell = grid.get(visual.x, visual.y);

        sprite.color = if cell.control > 0.0 {
            Color::rgb(0.2, 0.4, 1.0)
        } else {
            Color::rgb(1.0, 0.2, 0.2)
        };
    }
}

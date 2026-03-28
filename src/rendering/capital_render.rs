use crate::city::Capital;
use bevy::prelude::*;

#[derive(Component)]
pub struct CapitalVisual;

pub fn spawn_capitals(mut commands: Commands) {
    commands.spawn((
        Capital { faction: -1 },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.6, 0.1, 0.1),
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
            transform: Transform::from_xyz(-300.0, 0.0, 3.0),
            ..default()
        },
        CapitalVisual,
    ));

    commands.spawn((
        Capital { faction: 1 },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.6),
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
            transform: Transform::from_xyz(300.0, 0.0, 3.0),
            ..default()
        },
        CapitalVisual,
    ));

    println!("Capitals spawned");
}

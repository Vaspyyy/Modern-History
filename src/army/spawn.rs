use crate::army::Army;
use crate::core::GameConfig;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SpawnFaction {
    pub faction: i32,
}

pub fn spawn_initial_armies(mut commands: Commands, config: Res<GameConfig>) {
    let count = 5i32;

    for i in -count..=count {
        let offset_y = i as f32 * 50.0;

        commands.spawn(Army {
            position: Vec2::new(-250.0, offset_y),
            strength: config.initial_army_strength,
            faction: -1,
            speed: config.army_speed,
        });

        commands.spawn(Army {
            position: Vec2::new(250.0, offset_y),
            strength: config.initial_army_strength,
            faction: 1,
            speed: config.army_speed,
        });
    }

    info!("Spawned {} armies per faction", count * 2 + 1);
}

pub fn spawn_army_on_click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut spawn_faction: ResMut<SpawnFaction>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    config: Res<GameConfig>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        spawn_faction.faction = -1;
        debug!("Spawn faction set to: red (-1)");
    }

    if keys.just_pressed(KeyCode::Digit2) {
        spawn_faction.faction = 1;
        debug!("Spawn faction set to: blue (+1)");
    }

    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = cameras.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            commands.spawn(Army {
                position: world_position,
                strength: config.initial_army_strength,
                faction: spawn_faction.faction,
                speed: config.army_speed,
            });

            debug!(
                "Spawned army at ({:.1}, {:.1}) faction={}",
                world_position.x, world_position.y, spawn_faction.faction
            );
        }
    }
}

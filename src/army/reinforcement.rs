use crate::army::Army;
use crate::city::Capital;
use crate::core::GameConfig;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ReinforceTimer(pub Timer);

fn capital_position(faction: i32) -> Vec2 {
    match faction {
        -1 => Vec2::new(-300.0, 0.0),
        1 => Vec2::new(300.0, 0.0),
        _ => Vec2::ZERO,
    }
}

pub fn reinforce_from_capitals(
    mut commands: Commands,
    mut timer: ResMut<ReinforceTimer>,
    time: Res<Time>,
    capitals: Query<&Capital>,
    armies: Query<&Army>,
    config: Res<GameConfig>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    let army_count_by_faction =
        |faction: i32| -> usize { armies.iter().filter(|a| a.faction == faction).count() };

    let elapsed = timer.0.elapsed().as_secs_f32();

    for (i, capital) in capitals.iter().enumerate() {
        let faction = capital.faction;

        if army_count_by_faction(faction) >= config.max_armies_per_faction {
            continue;
        }

        let faction_phase = i as f32 * config.stagger_interval;
        let cycle = elapsed % (config.stagger_interval * 2.0);

        if cycle < faction_phase {
            continue;
        }

        let cap_pos = capital_position(faction);
        let count = army_count_by_faction(faction) as f32;
        let offset = Vec2::new(
            0.0,
            (count - config.max_armies_per_faction as f32 / 2.0) * config.army_spacing,
        );

        commands.spawn(Army {
            position: cap_pos + offset,
            strength: config.reinforce_strength,
            faction,
            speed: config.reinforce_speed,
        });
    }
}

use crate::army::Army;
use crate::city::Capital;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ReinforceTimer(pub Timer);

const MAX_ARMIES_PER_FACTION: usize = 15;
const REINFORCE_STRENGTH: f32 = 3000.0;
const REINFORCE_SPEED: f32 = 8.0;
const ARMY_SPACING: f32 = 20.0;
const STAGGER_INTERVAL: f32 = 2.5;

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
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    let army_count_by_faction =
        |faction: i32| -> usize { armies.iter().filter(|a| a.faction == faction).count() };

    let elapsed = timer.0.elapsed().as_secs_f32();

    for (i, capital) in capitals.iter().enumerate() {
        let faction = capital.faction;

        if army_count_by_faction(faction) >= MAX_ARMIES_PER_FACTION {
            continue;
        }

        let faction_phase = i as f32 * STAGGER_INTERVAL;
        let cycle = elapsed % (STAGGER_INTERVAL * 2.0);

        if cycle < faction_phase {
            continue;
        }

        let cap_pos = capital_position(faction);
        let count = army_count_by_faction(faction) as f32;
        let offset = Vec2::new(
            0.0,
            (count - MAX_ARMIES_PER_FACTION as f32 / 2.0) * ARMY_SPACING,
        );

        commands.spawn(Army {
            position: cap_pos + offset,
            strength: REINFORCE_STRENGTH,
            faction,
            speed: REINFORCE_SPEED,
        });
    }
}

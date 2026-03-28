use crate::army::Army;
use crate::core::GameConfig;
use bevy::prelude::*;

pub fn apply_repulsion(mut armies: Query<(Entity, &mut Army)>, config: Res<GameConfig>) {
    let radius = config.repulsion_radius;
    let strength = config.repulsion_strength;

    let army_data: Vec<(Entity, Vec2, i32)> = armies
        .iter()
        .map(|(entity, army)| (entity, army.position, army.faction))
        .collect();

    for i in 0..army_data.len() {
        for j in (i + 1)..army_data.len() {
            let (_, pos_a, faction_a) = army_data[i];
            let (_, pos_b, faction_b) = army_data[j];

            if faction_a == faction_b {
                continue;
            }

            let diff = pos_a - pos_b;
            let distance = diff.length();

            if distance < radius && distance > 0.001 {
                let factor = (1.0 - distance / radius) * strength;
                let push = diff.normalize_or_zero() * factor;

                if let Ok((_, mut army_a)) = armies.get_mut(army_data[i].0) {
                    army_a.position += push;
                }
                if let Ok((_, mut army_b)) = armies.get_mut(army_data[j].0) {
                    army_b.position -= push;
                }
            }
        }
    }
}

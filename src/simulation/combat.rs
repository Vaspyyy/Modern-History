use crate::army::Army;
use crate::core::GameConfig;
use bevy::prelude::*;

pub fn apply_combat(
    mut commands: Commands,
    mut armies: Query<(Entity, &mut Army)>,
    config: Res<GameConfig>,
) {
    let combat_radius = config.combat_radius;
    let damage_multiplier = config.damage_multiplier;
    let despawn_threshold = config.min_army_strength * 0.5;

    let army_data: Vec<(Entity, Vec2, f32, i32)> = armies
        .iter()
        .map(|(entity, army)| (entity, army.position, army.strength, army.faction))
        .collect();

    for (entity, mut army) in &mut armies {
        let mut total_damage = 0.0;

        for (other_entity, other_pos, other_strength, other_faction) in &army_data {
            if entity != *other_entity && army.faction != *other_faction {
                let distance = army.position.distance(*other_pos);

                if distance < combat_radius {
                    let damage = other_strength * damage_multiplier;
                    total_damage += damage;
                }
            }
        }

        army.strength -= total_damage;

        if army.strength <= despawn_threshold {
            commands.entity(entity).despawn();
        }
    }
}

use crate::army::Army;
use bevy::prelude::*;

pub fn apply_combat(mut commands: Commands, mut armies: Query<(Entity, &mut Army)>) {
    let combat_radius = 40.0;
    let damage_multiplier = 0.0005;

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

        if army.strength <= 50.0 {
            commands.entity(entity).despawn();
        }
    }
}

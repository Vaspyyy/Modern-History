use crate::army::Army;
use bevy::prelude::*;

const MERGE_RADIUS: f32 = 10.0;
const MAX_ARMY_STRENGTH: f32 = 20000.0;

pub fn consolidate_armies(mut commands: Commands, mut armies: Query<(Entity, &mut Army)>) {
    let army_data: Vec<(Entity, Vec2, f32, i32)> = armies
        .iter()
        .map(|(entity, army)| (entity, army.position, army.strength, army.faction))
        .collect();

    let mut to_despawn: Vec<Entity> = Vec::new();

    for i in 0..army_data.len() {
        if to_despawn.contains(&army_data[i].0) {
            continue;
        }

        for j in (i + 1)..army_data.len() {
            if to_despawn.contains(&army_data[j].0) {
                continue;
            }

            let (entity_a, pos_a, strength_a, faction_a) = army_data[i];
            let (entity_b, pos_b, strength_b, faction_b) = army_data[j];

            if faction_a != faction_b {
                continue;
            }

            if pos_a.distance(pos_b) > MERGE_RADIUS {
                continue;
            }

            let (survivor_entity, absorb_entity, absorb_strength) = if strength_a >= strength_b {
                (entity_a, entity_b, strength_b)
            } else {
                (entity_b, entity_a, strength_a)
            };

            if let Ok(mut survivor) = armies.get_mut(survivor_entity) {
                let new_strength = (survivor.1.strength + absorb_strength).min(MAX_ARMY_STRENGTH);
                survivor.1.strength = new_strength;
            }

            to_despawn.push(absorb_entity);
        }
    }

    for entity in to_despawn {
        commands.entity(entity).despawn();
    }
}

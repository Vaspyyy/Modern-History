use crate::ai::{ArmyOrder, Flanking};
use crate::army::Army;
use bevy::prelude::*;

const ARRIVAL_THRESHOLD: f32 = 5.0;

pub fn move_armies(
    mut query: Query<(&mut Army, &mut ArmyOrder, Option<&Flanking>)>,
    time: Res<Time>,
) {
    for (mut army, mut order, flanking) in &mut query {
        if let Some(flank) = flanking {
            let dist_to_order = army.position.distance(order.target);
            if dist_to_order < ARRIVAL_THRESHOLD {
                order.target = flank.target;
            }
        }

        let distance = army.position.distance(order.target);

        if distance < ARRIVAL_THRESHOLD {
            continue;
        }

        let dir = (order.target - army.position).normalize_or_zero();
        let speed = army.speed;
        army.position += dir * speed * time.delta_seconds();
    }
}

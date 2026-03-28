use crate::ai::{ArmyOrder, Flanking};
use crate::army::Army;
use crate::core::GameConfig;
use bevy::prelude::*;

pub fn move_armies(
    mut query: Query<(&mut Army, &mut ArmyOrder, Option<&Flanking>)>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    let arrival = config.arrival_threshold;

    for (mut army, mut order, flanking) in &mut query {
        if let Some(flank) = flanking {
            let dist_to_order = army.position.distance(order.target);
            if dist_to_order < arrival {
                order.target = flank.target;
            }
        }

        let distance = army.position.distance(order.target);

        if distance < arrival {
            continue;
        }

        let dir = (order.target - army.position).normalize_or_zero();
        let speed = army.speed;
        army.position += dir * speed * time.delta_seconds();
    }
}

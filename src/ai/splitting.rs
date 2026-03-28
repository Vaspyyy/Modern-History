use crate::ai::ArmyOrder;
use crate::army::Army;
use crate::core::GameConfig;
use crate::map::grid::Grid;
use crate::simulation::detect_frontline;
use bevy::prelude::*;

const MIN_FRONTLINE_CLUSTERS: usize = 3;

#[derive(Resource)]
pub struct SplitTimer(pub Timer);

fn find_farthest_frontline(army_pos: Vec2, frontline: &[Vec2], avoid: Vec2) -> Vec2 {
    let mut best_dist = 0.0;
    let mut best_point = Vec2::ZERO;

    for &point in frontline {
        let dist_to_avoid = point.distance(avoid);
        let dist_to_army = point.distance(army_pos);

        if dist_to_avoid > best_dist && dist_to_army > 50.0 {
            best_dist = dist_to_avoid;
            best_point = point;
        }
    }

    if best_dist == 0.0 && !frontline.is_empty() {
        let mut max_dist = 0.0;
        for &point in frontline {
            let d = point.distance(army_pos);
            if d > max_dist {
                max_dist = d;
                best_point = point;
            }
        }
    }

    best_point
}

fn frontline_spread(frontline: &[Vec2]) -> f32 {
    if frontline.len() < 2 {
        return 0.0;
    }

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for point in frontline {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    (max_x - min_x).max(max_y - min_y)
}

pub fn ai_split_armies(
    mut commands: Commands,
    mut armies: Query<(Entity, &mut Army, &ArmyOrder)>,
    grid: Res<Grid>,
    mut timer: ResMut<SplitTimer>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    let frontline = detect_frontline(&grid, config.cell_size);
    if frontline.len() < MIN_FRONTLINE_CLUSTERS {
        return;
    }

    let spread = frontline_spread(&frontline);
    if spread < 100.0 {
        return;
    }

    let faction_counts: std::collections::HashMap<i32, usize> = {
        let mut counts = std::collections::HashMap::new();
        for (_, army, _) in armies.iter() {
            *counts.entry(army.faction).or_insert(0) += 1;
        }
        counts
    };

    for (entity, mut army, order) in &mut armies {
        if army.strength < config.split_threshold {
            continue;
        }

        let count = *faction_counts.get(&army.faction).unwrap_or(&0);
        if count >= 30 {
            continue;
        }

        let split_strength = army.strength * config.split_ratio;
        army.strength -= split_strength;

        let new_target = find_farthest_frontline(army.position, &frontline, order.target);

        let split_offset = Vec2::new(
            (entity.index() as f32 % 7.0 - 3.0) * 5.0,
            (entity.index() as f32 % 5.0 - 2.0) * 5.0,
        );

        commands.spawn((
            Army {
                position: army.position + split_offset,
                strength: split_strength,
                faction: army.faction,
                speed: army.speed,
            },
            ArmyOrder {
                target: new_target,
                retreating: false,
            },
        ));
    }
}

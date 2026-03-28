use crate::ai::components::{ArmyOrder, DefendingBreakthrough};
use crate::army::Army;
use crate::map::grid::Grid;
use crate::simulation::GridHistory;
use bevy::prelude::*;

const DEFEND_RADIUS: f32 = 80.0;
const MIN_DEFENDER_STRENGTH: f32 = 1000.0;
const CELL_SIZE: f32 = 3.0;

struct ThreatCell {
    world_pos: Vec2,
    score: f32,
}

pub fn defend_breakthroughs(
    mut commands: Commands,
    mut armies: Query<(Entity, &Army, &mut ArmyOrder)>,
    grid: Res<Grid>,
    history: Res<GridHistory>,
) {
    let half_w = grid.width as f32 * CELL_SIZE / 2.0;
    let half_h = grid.height as f32 * CELL_SIZE / 2.0;

    let control_delta = history.control_delta(&grid);

    let mut threats_by_faction: std::collections::HashMap<i32, Vec<ThreatCell>> =
        std::collections::HashMap::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let idx = y * grid.width + x;
            let cell = grid.cells[idx];
            let delta = control_delta[idx];

            if cell.control.abs() < 0.2 && delta.abs() < 0.001 {
                continue;
            }

            let world_x = x as f32 * CELL_SIZE - half_w;
            let world_y = y as f32 * CELL_SIZE - half_h;
            let pos = Vec2::new(world_x, world_y);

            let target_faction: i32 = if cell.control < 0.0 { 1 } else { -1 };

            let is_enemy_territory = if target_faction == 1 {
                cell.control < -0.1
            } else {
                cell.control > 0.1
            };

            let is_pushing_toward = if target_faction == 1 {
                delta < -0.001
            } else {
                delta > 0.001
            };

            if !is_enemy_territory && !is_pushing_toward {
                continue;
            }

            let depth_score = if is_enemy_territory {
                cell.control.abs() * 0.3
            } else {
                0.0
            };

            let velocity_score = if is_pushing_toward {
                (delta.abs() * 10.0).min(1.0) * 0.5
            } else {
                0.0
            };

            let score = depth_score + velocity_score;

            if score > 0.05 {
                threats_by_faction
                    .entry(target_faction)
                    .or_default()
                    .push(ThreatCell {
                        world_pos: pos,
                        score,
                    });
            }
        }
    }

    for (_, threats) in threats_by_faction.iter_mut() {
        threats.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }

    let army_data: Vec<(Entity, Vec2, f32, i32)> = armies
        .iter()
        .map(|(e, a, _order)| (e, a.position, a.strength, a.faction))
        .collect();

    for (entity, army, mut order) in &mut armies {
        if army.strength < MIN_DEFENDER_STRENGTH {
            commands.entity(entity).remove::<DefendingBreakthrough>();
            continue;
        }

        if order.retreating {
            commands.entity(entity).remove::<DefendingBreakthrough>();
            continue;
        }

        let threats = match threats_by_faction.get(&army.faction) {
            Some(t) => t,
            None => {
                commands.entity(entity).remove::<DefendingBreakthrough>();
                continue;
            }
        };

        if threats.is_empty() {
            commands.entity(entity).remove::<DefendingBreakthrough>();
            continue;
        }

        let mut best_threat = Vec2::ZERO;
        let mut best_score = 0.0;
        let mut best_dist = f32::MAX;

        for threat in threats.iter().take(20) {
            let d = army.position.distance(threat.world_pos);
            if d > DEFEND_RADIUS * 4.0 {
                continue;
            }

            let nearby_enemy_strength: f32 = army_data
                .iter()
                .filter(|(_, _pos, _strength, faction)| *faction != army.faction)
                .filter(|(_, pos, _, _)| pos.distance(threat.world_pos) < 80.0)
                .map(|(_, _, s, _)| s)
                .sum();

            let army_bonus = nearby_enemy_strength * 0.0002;
            let distance_decay = 1.0 / (1.0 + d * 0.005);
            let total_score = (threat.score + army_bonus) * distance_decay;

            if total_score > best_score || (total_score > best_score * 0.9 && d < best_dist) {
                best_score = total_score;
                best_threat = threat.world_pos;
                best_dist = d;
            }
        }

        if best_score > 0.05 {
            commands.entity(entity).try_insert(DefendingBreakthrough);
            order.target = best_threat;
        } else {
            commands.entity(entity).remove::<DefendingBreakthrough>();
        }
    }
}

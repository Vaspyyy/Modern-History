use crate::ai::components::{ArmyOrder, Flanking};
use crate::army::Army;
use crate::core::GameConfig;
use bevy::prelude::*;

struct Salient {
    tip: Vec2,
    base_left: Vec2,
    base_right: Vec2,
    depth: f32,
    enemy_strength: f32,
}

fn detect_salients(
    frontline: &[Vec2],
    faction: i32,
    armies: &Query<(Entity, &Army)>,
    config: &GameConfig,
) -> Vec<Salient> {
    if frontline.len() < 5 {
        return Vec::new();
    }

    let mut sorted: Vec<Vec2> = frontline.to_vec();
    sorted.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let n = sorted.len();
    let window = (n as f32 * 0.15) as usize;
    if window < 2 {
        return Vec::new();
    }

    let mut expected_frontline: Vec<f32> = Vec::with_capacity(n);
    for i in 0..n {
        let start = i.saturating_sub(window);
        let end = (i + window + 1).min(n);
        let avg_x: f32 = sorted[start..end].iter().map(|p| p.x).sum::<f32>() / (end - start) as f32;
        expected_frontline.push(avg_x);
    }

    let enemy_armies: Vec<(Vec2, f32)> = armies
        .iter()
        .filter(|(_, a)| a.faction != faction)
        .map(|(_, a)| (a.position, a.strength))
        .collect();

    let mut deviations: Vec<(usize, f32)> = Vec::new();
    for i in 0..n {
        let deviation = if faction == 1 {
            expected_frontline[i] - sorted[i].x
        } else {
            sorted[i].x - expected_frontline[i]
        };

        if deviation > config.salient_min_depth {
            deviations.push((i, deviation));
        }
    }

    if deviations.is_empty() {
        return Vec::new();
    }

    let mut visited: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut salients: Vec<Salient> = Vec::new();

    for &(start_idx, start_dev) in &deviations {
        if visited.contains(&start_idx) {
            continue;
        }

        let mut tip = sorted[start_idx];
        let mut max_dev = start_dev;
        let mut min_idx = start_idx;
        let mut max_idx = start_idx;

        let mut j = start_idx + 1;
        while j < n && j < start_idx + window * 2 {
            let current_dev = if faction == 1 {
                expected_frontline[j] - sorted[j].x
            } else {
                sorted[j].x - expected_frontline[j]
            };

            if current_dev > config.salient_min_depth * 0.5 {
                visited.insert(j);
                max_idx = j;
                if current_dev > max_dev {
                    max_dev = current_dev;
                    tip = sorted[j];
                }
            } else {
                break;
            }
            j += 1;
        }

        j = start_idx.saturating_sub(1);
        while j > 0 && start_idx - j < window * 2 {
            let current_dev = if faction == 1 {
                expected_frontline[j] - sorted[j].x
            } else {
                sorted[j].x - expected_frontline[j]
            };

            if current_dev > config.salient_min_depth * 0.5 {
                visited.insert(j);
                min_idx = j;
                if current_dev > max_dev {
                    max_dev = current_dev;
                    tip = sorted[j];
                }
            } else {
                break;
            }
            j -= 1;
        }

        visited.insert(start_idx);

        let base_left = sorted[min_idx.min(start_idx)];
        let base_right = sorted[max_idx.max(start_idx)];

        let salient_center = Vec2::new(
            (base_left.x + base_right.x + tip.x) / 3.0,
            (base_left.y + base_right.y + tip.y) / 3.0,
        );

        let enemy_str: f32 = enemy_armies
            .iter()
            .filter(|(pos, _)| pos.distance(salient_center) < 100.0)
            .map(|(_, s)| s)
            .sum();

        if enemy_str < config.salient_min_enemy_strength {
            continue;
        }

        let depth = if faction == 1 {
            (expected_frontline[start_idx] - tip.x).max(0.0)
        } else {
            (tip.x - expected_frontline[start_idx]).max(0.0)
        };

        if depth < config.salient_min_depth {
            continue;
        }

        salients.push(Salient {
            tip,
            base_left,
            base_right,
            depth,
            enemy_strength: enemy_str,
        });
    }

    salients.sort_by(|a, b| {
        (b.depth * b.enemy_strength)
            .partial_cmp(&(a.depth * a.enemy_strength))
            .unwrap()
    });

    salients.truncate(3);
    salients
}

fn local_force_ratio_at(
    pos: Vec2,
    faction: i32,
    armies: &Query<(Entity, &Army)>,
    radius: f32,
) -> f32 {
    let mut friendly: f32 = 0.0;
    let mut enemy: f32 = 0.0;

    for (_, army) in armies.iter() {
        if pos.distance(army.position) < radius {
            if army.faction == faction {
                friendly += army.strength;
            } else {
                enemy += army.strength;
            }
        }
    }

    if enemy == 0.0 {
        return f32::INFINITY;
    }
    friendly / enemy
}

#[derive(Resource)]
pub struct FlankTimer(pub Timer);

pub fn assign_flanking_orders(
    mut commands: Commands,
    mut armies: Query<(Entity, &Army, &mut ArmyOrder, Option<&Flanking>)>,
    all_armies: Query<(Entity, &Army)>,
    mut timer: ResMut<FlankTimer>,
    time: Res<Time>,
    cached_frontline: Res<super::CachedFrontline>,
    config: Res<GameConfig>,
) {
    let ticked = timer.0.tick(time.delta()).just_finished();

    if !ticked {
        if cached_frontline.0.len() < 5 {
            for (entity, _, _, _) in &mut armies {
                commands.entity(entity).remove::<Flanking>();
            }
        }
        return;
    }

    let frontline = &cached_frontline.0;

    for (entity, _, _, flanking) in &mut armies {
        if flanking.is_some() {
            commands.entity(entity).remove::<Flanking>();
        }
    }

    if frontline.len() < 5 {
        return;
    }

    let mut faction_salients: std::collections::HashMap<i32, Vec<Salient>> =
        std::collections::HashMap::new();

    let factions = [-1, 1];
    for &faction in &factions {
        let sals = detect_salients(&frontline, faction, &all_armies, &config);
        if !sals.is_empty() {
            faction_salients.insert(faction, sals);
        }
    }

    let mut assigned: std::collections::HashSet<Entity> = std::collections::HashSet::new();

    for (entity, army, mut order, _) in &mut armies {
        if order.retreating {
            continue;
        }

        let salients = match faction_salients.get(&army.faction) {
            Some(s) => s,
            None => continue,
        };

        if assigned.contains(&entity) {
            continue;
        }

        let ratio = local_force_ratio_at(
            army.position,
            army.faction,
            &all_armies,
            config.flanker_radius,
        );
        if ratio < config.flank_force_ratio_threshold {
            continue;
        }

        let mut best_salient: Option<&Salient> = None;
        let mut best_score = 0.0;

        for salient in salients {
            let dist_to_base = army
                .position
                .distance(salient.base_left)
                .min(army.position.distance(salient.base_right));

            if dist_to_base > config.flanker_radius * 2.0 {
                continue;
            }

            let score =
                salient.depth * salient.enemy_strength * 0.001 / (1.0 + dist_to_base * 0.01);

            if score > best_score {
                best_score = score;
                best_salient = Some(salient);
            }
        }

        if let Some(salient) = best_salient {
            let dist_left = army.position.distance(salient.base_left);
            let dist_right = army.position.distance(salient.base_right);

            let (base, side) = if dist_left <= dist_right {
                (salient.base_left, crate::ai::components::FlankSide::Left)
            } else {
                (salient.base_right, crate::ai::components::FlankSide::Right)
            };

            let salient_dir =
                (salient.tip - (salient.base_left + salient.base_right) / 2.0).normalize_or_zero();
            let behind_tip = salient.tip + salient_dir * config.flank_offset;

            let to_target = (behind_tip - army.position).normalize_or_zero();
            let approach_target = base + to_target * 20.0;

            order.target = approach_target;
            commands.entity(entity).try_insert(Flanking {
                target: behind_tip,
                side,
            });
            assigned.insert(entity);
        }
    }
}

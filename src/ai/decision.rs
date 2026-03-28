use crate::ai::{ArmyOrder, DefendingBreakthrough, Flanking};
use crate::army::Army;
use crate::city::Capital;
use crate::map::grid::Grid;
use crate::simulation::detect_frontline;
use bevy::prelude::*;

const COMBAT_RADIUS: f32 = 40.0;
const STRENGTH_CHECK_RADIUS: f32 = 80.0;
const RETREAT_STRENGTH: f32 = 500.0;
const RECOVER_STRENGTH: f32 = 1500.0;
const MIN_CONSOLIDATE_GROUP: f32 = 4000.0;
const SUPPLY_RANGE: f32 = 200.0;
const NUM_SECTORS: usize = 5;

fn find_closest_point(army_pos: Vec2, points: &[Vec2]) -> Vec2 {
    let mut closest_dist = f32::MAX;
    let mut closest_point = Vec2::ZERO;

    for &point in points {
        let dist = army_pos.distance(point);
        if dist < closest_dist {
            closest_dist = dist;
            closest_point = point;
        }
    }

    closest_point
}

fn nearest_friendly_capital(army: &Army, capitals: &Query<(&Capital, &Transform)>) -> Vec2 {
    let mut best = Vec2::ZERO;
    let mut best_dist = f32::MAX;
    for (cap, transform) in capitals.iter() {
        if cap.faction == army.faction {
            let pos = transform.translation.truncate();
            let d = army.position.distance(pos);
            if d < best_dist {
                best_dist = d;
                best = pos;
            }
        }
    }
    best
}

fn nearest_friendly_army_pos(
    army: &Army,
    armies: &Query<(Entity, &Army)>,
    self_entity: Entity,
) -> Option<Vec2> {
    let mut best = None;
    let mut best_dist = f32::MAX;
    for (entity, other) in armies.iter() {
        if entity == self_entity {
            continue;
        }
        if other.faction == army.faction {
            let d = army.position.distance(other.position);
            if d < best_dist {
                best_dist = d;
                best = Some(other.position);
            }
        }
    }
    best
}

fn local_force_ratio(army: &Army, armies: &Query<(Entity, &Army)>, self_entity: Entity) -> f32 {
    let mut friendly: f32 = 0.0;
    let mut enemy: f32 = 0.0;

    for (entity, other) in armies.iter() {
        if entity == self_entity {
            continue;
        }
        if army.position.distance(other.position) < STRENGTH_CHECK_RADIUS {
            if other.faction == army.faction {
                friendly += other.strength;
            } else {
                enemy += other.strength;
            }
        }
    }

    if enemy == 0.0 {
        return f32::INFINITY;
    }
    (army.strength + friendly) / enemy
}

fn nearest_enemy_pos(
    army: &Army,
    armies: &Query<(Entity, &Army)>,
    self_entity: Entity,
) -> Option<Vec2> {
    let mut best = None;
    let mut best_dist = f32::MAX;
    for (entity, other) in armies.iter() {
        if entity == self_entity {
            continue;
        }
        if other.faction != army.faction {
            let d = army.position.distance(other.position);
            if d < best_dist {
                best_dist = d;
                best = Some(other.position);
            }
        }
    }
    best
}

struct FrontlineSectors {
    sector_centers: Vec<Vec2>,
    sector_enemy_pressure: Vec<f32>,
}

fn compute_frontline_sectors(
    frontline: &[Vec2],
    faction: i32,
    armies: &Query<(Entity, &Army)>,
    grid: &Grid,
) -> FrontlineSectors {
    if frontline.len() < 2 {
        return FrontlineSectors {
            sector_centers: vec![find_closest_point(Vec2::ZERO, frontline)],
            sector_enemy_pressure: vec![1.0],
        };
    }

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    for p in frontline {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
    }

    let n = NUM_SECTORS.min(frontline.len());
    let step = (max_x - min_x) / n as f32;

    let mut sector_centers: Vec<Vec2> = Vec::with_capacity(n);
    let mut sector_points: Vec<Vec<Vec2>> = Vec::new();
    sector_points.resize(n, Vec::new());
    let mut sector_enemy_pressure: Vec<f32> = vec![0.0; n];

    let cell_size = 3.0;
    let half_w = grid.width as f32 * cell_size / 2.0;
    let half_h = grid.height as f32 * cell_size / 2.0;

    let mut enemy_strength_near_sector: Vec<f32> = vec![0.0; n];

    for (_entity, other) in armies.iter() {
        if other.faction == faction {
            continue;
        }
        for (i, center) in sector_centers.iter().enumerate().take(n) {
            let d = other.position.distance(*center);
            if d < 100.0 {
                enemy_strength_near_sector[i] += other.strength;
            }
        }
    }

    for &p in frontline {
        let idx = ((p.x - min_x) / step) as usize;
        let idx = idx.min(n - 1);
        sector_points[idx].push(p);
    }

    for i in 0..n {
        if sector_points[i].is_empty() {
            sector_centers.push(Vec2::ZERO);
            sector_enemy_pressure.push(0.0);
            continue;
        }

        let mut cx = 0.0;
        let mut cy = 0.0;
        for p in &sector_points[i] {
            cx += p.x;
            cy += p.y;
        }
        cx /= sector_points[i].len() as f32;
        cy /= sector_points[i].len() as f32;
        sector_centers.push(Vec2::new(cx, cy));
    }

    let mut max_pressure: f32 = 0.0;
    for i in 0..n {
        let cell_x = ((sector_centers[i].x + half_w) / cell_size) as isize;
        let cell_y = ((sector_centers[i].y + half_h) / cell_size) as isize;
        let mut control_at_front = 0.0;
        if cell_x >= 0
            && cell_x < grid.width as isize
            && cell_y >= 0
            && cell_y < grid.height as isize
        {
            control_at_front = grid.get(cell_x as usize, cell_y as usize).control;
        }

        let enemy_control = if faction == 1 {
            (-control_at_front).max(0.0)
        } else {
            control_at_front.max(0.0)
        };

        let pressure = enemy_control + enemy_strength_near_sector[i] * 0.0001;
        sector_enemy_pressure.push(pressure);
        max_pressure = max_pressure.max(pressure);
    }

    if max_pressure > 0.0 {
        for p in sector_enemy_pressure.iter_mut() {
            *p /= max_pressure;
        }
    } else {
        for p in sector_enemy_pressure.iter_mut() {
            *p = 1.0 / n as f32;
        }
    }

    FrontlineSectors {
        sector_centers,
        sector_enemy_pressure,
    }
}

fn select_sector_target(
    army: &Army,
    sectors: &FrontlineSectors,
    faction_counts: &std::collections::HashMap<i32, Vec<Vec2>>,
    frontline: &[Vec2],
) -> Vec2 {
    let friendly_positions = match faction_counts.get(&army.faction) {
        Some(p) => p,
        None => return find_closest_point(army.position, &sectors.sector_centers),
    };

    let n = sectors.sector_centers.len();
    let mut sector_load: Vec<f32> = vec![0.0; n];

    for &pos in friendly_positions {
        let mut min_d = f32::MAX;
        let mut min_idx = 0;
        for (i, center) in sectors.sector_centers.iter().enumerate() {
            let d = pos.distance(*center);
            if d < min_d {
                min_d = d;
                min_idx = i;
            }
        }
        if min_d < 120.0 {
            sector_load[min_idx] += 1.0;
        }
    }

    let mut best_score = f32::MIN;
    let mut best_sector = find_closest_point(army.position, &sectors.sector_centers);
    let mut best_idx = 0;

    for (i, center) in sectors.sector_centers.iter().enumerate() {
        if center == &Vec2::ZERO {
            continue;
        }

        let threat_weight = sectors.sector_enemy_pressure[i];
        let overload_penalty = sector_load[i] * 0.3;
        let distance_factor = 1.0 / (1.0 + army.position.distance(*center) * 0.005);
        let score = threat_weight * 2.0 - overload_penalty + distance_factor;

        if score > best_score {
            best_score = score;
            best_sector = *center;
            best_idx = i;
        }
    }

    let mut closest_in_sector = Vec2::ZERO;
    let mut closest_d = f32::MAX;
    let target_x = sectors.sector_centers[best_idx].x;
    for &fl in frontline {
        if (fl.x - target_x).abs() < 60.0 {
            let d = army.position.distance(fl);
            if d < closest_d {
                closest_d = d;
                closest_in_sector = fl;
            }
        }
    }

    if closest_in_sector != Vec2::ZERO {
        closest_in_sector
    } else {
        best_sector
    }
}

fn retreat_waypoint(
    army: &Army,
    capital: Vec2,
    armies: &Query<(Entity, &Army)>,
    self_entity: Entity,
) -> Vec2 {
    let to_capital = (capital - army.position).normalize_or_zero();
    let perpendicular = Vec2::new(-to_capital.y, to_capital.x);
    let mut best_pos = capital;
    let mut best_enemy_dist = 0.0;

    for offset in [-40.0f32, 40.0, -80.0, 80.0, -120.0, 120.0] {
        let candidate = army.position + to_capital * 30.0 + perpendicular * offset;
        let mut min_enemy_d = f32::MAX;
        for (entity, other) in armies.iter() {
            if entity == self_entity {
                continue;
            }
            if other.faction != army.faction {
                min_enemy_d = min_enemy_d.min(candidate.distance(other.position));
            }
        }

        let candidate_to_cap = candidate.distance(capital);
        let score = min_enemy_d * 2.0 - candidate_to_cap;

        if score > best_enemy_dist && min_enemy_d > COMBAT_RADIUS {
            best_enemy_dist = score;
            best_pos = candidate;
        }
    }

    best_pos
}

fn choose_target(
    army: &Army,
    entity: Entity,
    frontline: &[Vec2],
    all_armies: &Query<(Entity, &Army)>,
    capitals: &Query<(&Capital, &Transform)>,
    faction_counts: &std::collections::HashMap<i32, Vec<Vec2>>,
    grid: &Grid,
) -> Vec2 {
    let force_ratio = local_force_ratio(army, all_armies, entity);

    if force_ratio < 1.2 && army.strength < MIN_CONSOLIDATE_GROUP {
        if let Some(friendly_pos) = nearest_friendly_army_pos(army, all_armies, entity) {
            return friendly_pos;
        }
    }

    if !frontline.is_empty() {
        let sectors = compute_frontline_sectors(frontline, army.faction, all_armies, grid);
        return select_sector_target(army, &sectors, faction_counts, frontline);
    }

    let capital = nearest_friendly_capital(army, capitals);
    let dist_to_cap = army.position.distance(capital);

    if dist_to_cap < SUPPLY_RANGE && army.strength < 3000.0 {
        return capital + Vec2::ZERO;
    }

    capital
}

fn build_faction_positions(
    armies: &Query<(Entity, &Army)>,
) -> std::collections::HashMap<i32, Vec<Vec2>> {
    let mut map: std::collections::HashMap<i32, Vec<Vec2>> = std::collections::HashMap::new();
    for (_, army) in armies.iter() {
        map.entry(army.faction).or_default().push(army.position);
    }
    map
}

pub fn assign_new_orders(
    mut commands: Commands,
    armies: Query<(Entity, &Army), Without<ArmyOrder>>,
    grid: Res<Grid>,
    all_armies: Query<(Entity, &Army)>,
    capitals: Query<(&Capital, &Transform)>,
    mut cached_frontline: ResMut<super::CachedFrontline>,
) {
    let frontline = detect_frontline(&grid);
    cached_frontline.0 = frontline.clone();

    let faction_counts = build_faction_positions(&all_armies);

    for (entity, army) in &armies {
        let target = choose_target(
            army,
            entity,
            &frontline,
            &all_armies,
            &capitals,
            &faction_counts,
            &grid,
        );

        commands.entity(entity).try_insert(ArmyOrder {
            target,
            retreating: false,
        });
    }
}

pub fn assign_orders_timed(
    mut commands: Commands,
    armies_without_order: Query<(Entity, &Army), Without<ArmyOrder>>,
    mut armies_with_order: Query<
        (Entity, &Army, &mut ArmyOrder),
        (Without<DefendingBreakthrough>, Without<Flanking>),
    >,
    grid: Res<Grid>,
    mut timer: ResMut<crate::app::AITimer>,
    time: Res<Time>,
    all_armies: Query<(Entity, &Army)>,
    capitals: Query<(&Capital, &Transform)>,
    cached_frontline: Res<super::CachedFrontline>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    let frontline = &cached_frontline.0;

    let faction_counts = build_faction_positions(&all_armies);

    for (entity, army) in &armies_without_order {
        let target = choose_target(
            army,
            entity,
            frontline,
            &all_armies,
            &capitals,
            &faction_counts,
            &grid,
        );

        commands.entity(entity).try_insert(ArmyOrder {
            target,
            retreating: false,
        });
    }

    for (entity, army, mut order) in &mut armies_with_order {
        if order.retreating {
            if army.strength >= RECOVER_STRENGTH {
                order.retreating = false;
                if !frontline.is_empty() {
                    let sectors =
                        compute_frontline_sectors(frontline, army.faction, &all_armies, &grid);
                    order.target = select_sector_target(army, &sectors, &faction_counts, frontline);
                }
            } else {
                let capital = nearest_friendly_capital(army, &capitals);
                let wp = retreat_waypoint(army, capital, &all_armies, entity);
                order.target = wp;
            }
            continue;
        }

        let force_ratio = local_force_ratio(army, &all_armies, entity);

        if army.strength < RETREAT_STRENGTH && force_ratio < 1.0 {
            order.retreating = true;
            let capital = nearest_friendly_capital(army, &capitals);
            let wp = retreat_waypoint(army, capital, &all_armies, entity);
            order.target = wp;
            continue;
        }

        if force_ratio < 0.7 && army.strength < MIN_CONSOLIDATE_GROUP {
            if let Some(friendly_pos) = nearest_friendly_army_pos(army, &all_armies, entity) {
                order.target = friendly_pos;
                continue;
            }
        }

        if !frontline.is_empty() {
            let sectors = compute_frontline_sectors(frontline, army.faction, &all_armies, &grid);
            order.target = select_sector_target(army, &sectors, &faction_counts, frontline);
        }
    }
}

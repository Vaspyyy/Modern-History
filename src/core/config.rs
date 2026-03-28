use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,

    pub combat_radius: f32,
    pub damage_multiplier: f32,
    pub min_army_strength: f32,

    pub supply_range: f32,
    pub supply_heal_rate: f32,
    pub supply_attrition_rate: f32,

    pub control_speed: f32,

    pub initial_army_strength: f32,
    pub army_speed: f32,

    pub merge_radius: f32,
    pub max_army_strength: f32,

    pub arrival_threshold: f32,

    pub max_armies_per_faction: usize,
    pub reinforce_strength: f32,
    pub reinforce_speed: f32,
    pub army_spacing: f32,

    pub snapshot_interval: f32,

    pub ai_order_interval: f32,
    pub reinforce_interval: f32,
    pub split_interval: f32,
    pub flank_interval: f32,

    pub strength_check_radius: f32,
    pub retreat_strength: f32,
    pub recover_strength: f32,
    pub min_consolidate_group: f32,
    pub num_sectors: usize,

    pub defend_radius: f32,
    pub min_defender_strength: f32,

    pub split_threshold: f32,
    pub split_ratio: f32,

    pub salient_min_depth: f32,
    pub salient_min_enemy_strength: f32,
    pub flank_offset: f32,
    pub flank_force_ratio_threshold: f32,
    pub flanker_radius: f32,

    pub repulsion_radius: f32,
    pub repulsion_strength: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid_width: 256,
            grid_height: 256,
            cell_size: 3.0,

            combat_radius: 40.0,
            damage_multiplier: 0.0005,
            min_army_strength: 100.0,

            supply_range: 200.0,
            supply_heal_rate: 2.0,
            supply_attrition_rate: 1.0,

            control_speed: 0.0001,

            initial_army_strength: 5000.0,
            army_speed: 8.0,

            merge_radius: 10.0,
            max_army_strength: 20000.0,

            arrival_threshold: 5.0,

            max_armies_per_faction: 15,
            reinforce_strength: 3000.0,
            reinforce_speed: 8.0,
            army_spacing: 20.0,

            snapshot_interval: 1.0,

            ai_order_interval: 1.0,
            reinforce_interval: 10.0,
            split_interval: 5.0,
            flank_interval: 1.0,

            strength_check_radius: 80.0,
            retreat_strength: 500.0,
            recover_strength: 1500.0,
            min_consolidate_group: 4000.0,
            num_sectors: 5,

            defend_radius: 80.0,
            min_defender_strength: 1000.0,

            split_threshold: 10000.0,
            split_ratio: 0.4,

            salient_min_depth: 30.0,
            salient_min_enemy_strength: 2000.0,
            flank_offset: 50.0,
            flank_force_ratio_threshold: 1.5,
            flanker_radius: 120.0,

            repulsion_radius: 30.0,
            repulsion_strength: 5.0,
        }
    }
}

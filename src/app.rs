use crate::ai::ai_split_armies;
use crate::ai::assign_flanking_orders;
use crate::ai::assign_new_orders;
use crate::ai::assign_orders_timed;
use crate::ai::defend_breakthroughs;
use crate::ai::FlankTimer;
use crate::ai::SplitTimer;
use crate::army::consolidate_armies;
use crate::army::move_armies;
use crate::army::reinforce_from_capitals;
use crate::army::spawn_army_on_click;
use crate::army::spawn_initial_armies;
use crate::army::ReinforceTimer;
use crate::army::SpawnFaction;
use crate::map::grid::Grid;
use crate::map::setup_grid;
use crate::map::MapPlugin;
use crate::rendering::attach_army_visuals;
use crate::rendering::cleanup_orphan_army_text;
use crate::rendering::spawn_capitals;
use crate::rendering::spawn_grid_visuals;
use crate::rendering::update_army_text;
use crate::rendering::update_army_visuals;
use crate::rendering::update_grid_visuals;
use crate::simulation::apply_combat;
use crate::simulation::apply_pressure;
use crate::simulation::apply_supply;
use crate::simulation::snapshot_control;
use crate::simulation::update_control;
use crate::simulation::GridHistory;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AITimer(pub Timer);

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .insert_resource(SpawnFaction::default())
        .insert_resource(AITimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(ReinforceTimer(Timer::from_seconds(
            10.0,
            TimerMode::Repeating,
        )))
        .insert_resource(SplitTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .insert_resource(FlankTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(
            Startup,
            (
                setup,
                setup_grid,
                init_grid_history,
                spawn_capitals,
                spawn_grid_visuals,
                spawn_initial_armies,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            (
                cleanup_orphan_army_text,
                attach_army_visuals,
                update_army_visuals,
                update_army_text,
            ),
        )
        .add_systems(
            Update,
            (
                snapshot_control,
                consolidate_armies,
                apply_pressure,
                apply_supply,
                apply_combat,
                update_control,
                update_grid_visuals,
                assign_flanking_orders,
                assign_new_orders,
                assign_orders_timed,
                defend_breakthroughs,
                ai_split_armies,
                move_armies,
                reinforce_from_capitals,
                spawn_army_on_click,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    println!("Modern History simulation starting...");
    commands.spawn(Camera2dBundle::default());
}

fn init_grid_history(mut commands: Commands, grid: Res<Grid>) {
    commands.insert_resource(GridHistory::new(&grid));
}

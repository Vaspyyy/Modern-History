use crate::ai::ai_split_armies;
use crate::ai::assign_flanking_orders;
use crate::ai::assign_new_orders;
use crate::ai::assign_orders_timed;
use crate::ai::defend_breakthroughs;
use crate::ai::CachedFrontline;
use crate::ai::FlankTimer;
use crate::ai::SplitTimer;
use crate::army::consolidate_armies;
use crate::army::move_armies;
use crate::army::reinforce_from_capitals;
use crate::army::spawn_army_on_click;
use crate::army::spawn_initial_armies;
use crate::army::ReinforceTickCounter;
use crate::army::ReinforceTimer;
use crate::army::SpawnFaction;
use crate::core::GameConfig;
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimulationSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AISet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovementSet;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameConfig::default())
        .add_plugins(MapPlugin)
        .insert_resource(SpawnFaction::default())
        .insert_resource(CachedFrontline::default())
        .insert_resource(ReinforceTickCounter(0))
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
        .configure_sets(Update, (SimulationSet, AISet, MovementSet).chain())
        .add_systems(
            Update,
            (
                snapshot_control,
                consolidate_armies,
                (apply_pressure, apply_supply, apply_combat).chain(),
                update_control,
            )
                .chain()
                .in_set(SimulationSet),
        )
        .add_systems(
            Update,
            (
                assign_new_orders,
                assign_orders_timed,
                assign_flanking_orders,
                defend_breakthroughs,
                ai_split_armies,
            )
                .chain()
                .in_set(AISet),
        )
        .add_systems(
            Update,
            (move_armies, reinforce_from_capitals)
                .chain()
                .in_set(MovementSet),
        )
        .add_systems(Update, update_grid_visuals.after(SimulationSet))
        .add_systems(Update, spawn_army_on_click)
        .add_systems(
            PostUpdate,
            (
                cleanup_orphan_army_text,
                attach_army_visuals,
                update_army_visuals,
                update_army_text,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, config: Res<GameConfig>) {
    info!("Modern History simulation starting...");
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(AITimer(Timer::from_seconds(
        config.ai_order_interval,
        TimerMode::Repeating,
    )));
    commands.insert_resource(ReinforceTimer(Timer::from_seconds(
        config.reinforce_interval,
        TimerMode::Repeating,
    )));
    commands.insert_resource(SplitTimer(Timer::from_seconds(
        config.split_interval,
        TimerMode::Repeating,
    )));
    commands.insert_resource(FlankTimer(Timer::from_seconds(
        config.flank_interval,
        TimerMode::Repeating,
    )));
}

fn init_grid_history(mut commands: Commands, grid: Res<Grid>, config: Res<GameConfig>) {
    commands.insert_resource(GridHistory::new(&grid, config.snapshot_interval));
}

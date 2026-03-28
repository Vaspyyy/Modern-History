use crate::map::grid::Grid;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GridHistory {
    pub previous_control: Vec<f32>,
    timer: Timer,
}

impl GridHistory {
    pub fn new(grid: &Grid, snapshot_interval: f32) -> Self {
        Self {
            previous_control: grid.cells.iter().map(|c| c.control).collect(),
            timer: Timer::from_seconds(snapshot_interval, TimerMode::Repeating),
        }
    }

    pub fn control_delta(&self, grid: &Grid) -> Vec<f32> {
        grid.cells
            .iter()
            .zip(self.previous_control.iter())
            .map(|(cell, prev)| cell.control - prev)
            .collect()
    }
}

pub fn snapshot_control(mut history: ResMut<GridHistory>, grid: Res<Grid>, time: Res<Time>) {
    if history.timer.tick(time.delta()).finished() {
        history.previous_control.resize(grid.cells.len(), 0.0);
        for (i, cell) in grid.cells.iter().enumerate() {
            history.previous_control[i] = cell.control;
        }
    }
}

use bevy::prelude::*;

#[derive(Component)]
pub struct ArmyOrder {
    pub target: Vec2,
    pub retreating: bool,
}

#[derive(Component)]
pub struct DefendingBreakthrough;

#[derive(Component)]
pub struct Flanking {
    pub target: Vec2,
    pub side: FlankSide,
}

#[derive(Clone, Copy)]
pub enum FlankSide {
    Left,
    Right,
}

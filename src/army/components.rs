use bevy::prelude::*;

#[derive(Component)]
pub struct Army {
    pub position: Vec2,
    pub strength: f32,
    pub faction: i32, // -1 or +1
    pub speed: f32,   // units per second
}

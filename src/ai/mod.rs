pub mod components;
pub mod decision;
pub mod defense;
pub mod splitting;
pub mod strategy;
pub mod tactics;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CachedFrontline(pub Vec<Vec2>);

pub use components::{ArmyOrder, DefendingBreakthrough, Flanking};
pub use decision::{assign_new_orders, assign_orders_timed};
pub use defense::defend_breakthroughs;
pub use splitting::{ai_split_armies, SplitTimer};
pub use tactics::{assign_flanking_orders, FlankTimer};

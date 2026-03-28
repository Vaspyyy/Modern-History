pub mod components;
pub mod consolidation;
pub mod movement;
pub mod reinforcement;
pub mod spawn;

pub use components::Army;
pub use consolidation::consolidate_armies;
pub use movement::move_armies;
pub use reinforcement::{reinforce_from_capitals, ReinforceTimer};
pub use spawn::{spawn_army_on_click, spawn_initial_armies, SpawnFaction};

pub mod army_render;
pub mod capital_render;
pub mod map_render;

pub use army_render::{
    attach_army_visuals, cleanup_orphan_army_text, update_army_text, update_army_visuals,
};
pub use capital_render::spawn_capitals;
pub use map_render::{spawn_grid_visuals, update_grid_visuals};

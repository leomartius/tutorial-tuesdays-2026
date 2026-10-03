mod input;
mod render;
mod theme;

pub use input::map_play_command;
pub use render::render_entities;
pub use render::render_map;
pub use render::render_player;
pub use theme::Theme;

pub enum Command {
    Bump(i32, i32),
    Wait,
}

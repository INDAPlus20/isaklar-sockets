mod app;
mod game_data;
mod game_state;

use app::{AppState, SCREEN_SIZE};
use ggez::event;
use std::path;

fn main() {
    let resource_dir = path::PathBuf::from("./resources");
    let context_builder = ggez::ContextBuilder::new("tetris", "malte och isak")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Tetris goes brrr"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimenstions
                .resizable(true), // Fixate window size
        );

    let (contex, event_loop) = &mut context_builder.build().expect("context builder error");

    let state = &mut AppState::new(contex);

    event::run(contex, event_loop, state);
}

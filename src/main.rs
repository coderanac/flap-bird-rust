mod game;
use crate::game::GameState;
use ggez::event::{self};
use ggez::{ContextBuilder, GameResult};
use std::path::PathBuf;
const BIRD_IMAGE_BYTES: &'static [u8] = include_bytes!("../resources/bird.png");

fn main() -> GameResult {
    let resource_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(|manifest_dir| {
            let mut path = PathBuf::from(manifest_dir);
            path.push("resources");
            path
        })
        .unwrap_or_else(|_| PathBuf::from("."));

    let (mut ctx, event_loop) = ContextBuilder::new("flappy_bird", "Carol")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Flappy Bird!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let game_state = GameState::new(&mut ctx, BIRD_IMAGE_BYTES)?;

    event::run(ctx, event_loop, game_state)
}

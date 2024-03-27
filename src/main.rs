mod game;
use crate::game::GameState;
use ggez::event::{self};
use ggez::{ContextBuilder, GameResult};
const BIRD_IMAGE_BYTES: &'static [u8] = include_bytes!("../resources/bird.png");

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("flappy_bird", "Carol")
        .window_setup(ggez::conf::WindowSetup::default().title("Flappy Bird!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let game_state = GameState::new(&mut ctx, BIRD_IMAGE_BYTES)?;

    event::run(ctx, event_loop, game_state)
}

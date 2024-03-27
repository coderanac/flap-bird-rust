mod behavior;
mod controls;

use ggez::graphics::{Image, Rect};
use ggez::{Context, GameResult};

pub struct GameState {
    state: Game,
    bird_image: Image,
    bird_pos: [f32; 2],
    bird_velocity: f32,
    obstacles: Vec<ObstaclePair>,
    obstacle_spawn_timer: f32,
    score: i32,
}

impl GameState {
    pub fn new(ctx: &mut Context, bird_image_bytes: &'static [u8]) -> GameResult<GameState> {
        let bird_image = Image::from_bytes(ctx, bird_image_bytes)?;
        Ok(GameState {
            state: Game::Menu,
            bird_image,
            bird_pos: [50.0, 50.0],
            bird_velocity: 0.0,
            obstacles: Vec::new(),
            obstacle_spawn_timer: 0.0,
            score: 0,
        })
    }
}

#[derive(PartialEq)]
enum Game {
    Menu,
    Playing,
    GameOver,
}

struct ObstaclePair {
    top: Rect,
    bottom: Rect,
    passed: bool,
}

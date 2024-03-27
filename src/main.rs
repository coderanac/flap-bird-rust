mod game;
use ggez::event::{self};
use ggez::{ContextBuilder, GameResult};

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        Some(path)
    } else {
        None
    };

    let (mut ctx, event_loop) = ContextBuilder::new("flappy_bird", "Carol")
        .add_resource_path(resource_dir.unwrap())
        .window_setup(ggez::conf::WindowSetup::default().title("Flappy Bird!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = game::GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

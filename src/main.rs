use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, Image, Rect, Text};
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::GameError;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

struct ObstaclePair {
    top: Rect,
    bottom: Rect,
    passed: bool,
}

#[derive(PartialEq)]
enum Game {
    Menu,
    Playing,
    GameOver,
}

struct GameState {
    state: Game,
    bird_image: Image,
    bird_pos: [f32; 2],
    bird_velocity: f32,
    obstacles: Vec<ObstaclePair>,
    obstacle_spawn_timer: f32,
    score: i32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let bird_image = Image::new(ctx, "/bird.png")?;
        Ok(GameState {
            state: Game::Menu, // Começa no menu
            bird_image,
            bird_pos: [50.0, 50.0],
            bird_velocity: 0.0,
            obstacles: Vec::new(),
            obstacle_spawn_timer: 0.0,
            score: 0,
        })
    }

    fn add_obstacle(&mut self, ctx: &mut Context) {
        let mut rng = rand::thread_rng();
        let gap_y = rng.gen_range(200.0..400.0);
        let gap_height = 150.0;

        let top_height = gap_y - gap_height / 2.0;
        let bottom_y = gap_y + gap_height / 2.0;
        let bottom_height = 600.0 - bottom_y;

        self.obstacles.push(ObstaclePair {
            top: Rect::new(800.0, 0.0, 50.0, top_height),
            bottom: Rect::new(800.0, bottom_y, 50.0, bottom_height),
            passed: false,
        });
    }

    fn check_collision(&self) -> bool {
        let bird_scale = 0.15;
        let bird_width = self.bird_image.width() as f32 * bird_scale;
        let bird_height = self.bird_image.height() as f32 * bird_scale;

        let bird_rect = Rect::new(self.bird_pos[0], self.bird_pos[1], bird_width, bird_height);

        for obstacle in &self.obstacles {
            let top_rect = Rect::new(
                obstacle.top.x,
                obstacle.top.y,
                obstacle.top.w,
                obstacle.top.h,
            );
            let bottom_rect = Rect::new(
                obstacle.bottom.x,
                obstacle.bottom.y,
                obstacle.bottom.w,
                obstacle.bottom.h,
            );

            if bird_rect.overlaps(&top_rect) || bird_rect.overlaps(&bottom_rect) {
                return true;
            }
        }

        false
    }

    fn game_over(&mut self) {
        self.state = Game::GameOver;
    }

    fn score(&mut self) {
        let bird_x = self.bird_pos[0] + (self.bird_image.width() as f32 * 0.15); // Considerando a largura do pássaro após escala.
        for obstacle in &mut self.obstacles {
            if !obstacle.passed && bird_x > obstacle.top.x + obstacle.top.w {
                obstacle.passed = true;
                self.score += 1;
            }
        }
    }
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            Game::Playing => {
                self.bird_velocity += 0.5;
                self.bird_pos[1] += self.bird_velocity;

                if self.check_collision() {
                    self.game_over();
                }

                self.obstacle_spawn_timer += timer::delta(ctx).as_secs_f32();
                if self.obstacle_spawn_timer > 1.5 {
                    self.obstacle_spawn_timer = 0.0;
                    self.add_obstacle(ctx);
                }

                for obstacle in &mut self.obstacles {
                    obstacle.top.x -= 2.0;
                    obstacle.bottom.x -= 2.0;
                }

                self.obstacles
                    .retain(|obstacle| obstacle.top.x + obstacle.top.w > 0.0);

                self.score();
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        let score_text = Text::new(format!("Score: {}", self.score));
        let score_pos = [20.0, 20.0];
        graphics::draw(ctx, &score_text, (score_pos, 0.0, Color::BLACK))?;

        let bird_scale = graphics::DrawParam::default()
            .scale([0.15, 0.15])
            .dest([self.bird_pos[0], self.bird_pos[1]]);
        graphics::draw(ctx, &self.bird_image, bird_scale)?;

        for obstacle in &self.obstacles {
            let top_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                obstacle.top,
                Color::GREEN,
            )?;
            graphics::draw(ctx, &top_rect, graphics::DrawParam::default())?;

            let bottom_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                obstacle.bottom,
                Color::GREEN,
            )?;
            graphics::draw(ctx, &bottom_rect, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match self.state {
            Game::Menu => {
                if keycode == KeyCode::Return {
                    self.state = Game::Playing;
                }
            }
            Game::Playing => {
                if keycode == KeyCode::Space {
                    self.bird_velocity = -06.0;
                }
            }
            Game::GameOver => {
                if keycode == KeyCode::R {
                    self.restart(_ctx).expect("Falha ao reiniciar o jogo")
                }
            }
        }
    }
}

impl GameState {
    fn restart(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.state = Game::Playing;
        self.bird_pos = [50.0, 50.0];
        self.bird_velocity = 0.0;
        self.obstacles.clear();
        self.obstacle_spawn_timer = 0.0;
        self.score = 0;
        Ok(())
    }
}

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

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

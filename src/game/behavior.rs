use super::Game;
use super::GameState;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Color, Text};
use ggez::input::keyboard::KeyMods;
use ggez::timer;
use ggez::GameError;
use ggez::{Context, GameResult};

impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            Game::Playing => {
                self.bird_velocity += 0.5;
                self.bird_pos[1] += self.bird_velocity;

                if self.check_collision(ctx) {
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

        if self.state == Game::Menu {
            let start_text = Text::new("Press Enter to start");
            let start_pos = [320.0, 300.0];
            graphics::draw(ctx, &start_text, (start_pos, 0.0, Color::BLACK))?;
        }

        if self.state == Game::GameOver {
            let start_text = Text::new("Press R to restart");
            let start_pos = [320.0, 300.0];
            graphics::draw(ctx, &start_text, (start_pos, 0.0, Color::BLACK))?;
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

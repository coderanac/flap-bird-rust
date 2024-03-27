use super::Game;
use super::GameState;
use super::ObstaclePair;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use rand::Rng;

impl GameState {
    pub fn add_obstacle(&mut self, _ctx: &mut Context) {
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

    pub fn check_collision(&self, ctx: &Context) -> bool {
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

        if bird_rect.top() < 0.0 || bird_rect.bottom() > graphics::drawable_size(ctx).1 {
            return true;
        }

        false
    }

    pub fn game_over(&mut self) {
        self.state = Game::GameOver;
    }

    pub fn score(&mut self) {
        let bird_x = self.bird_pos[0] + (self.bird_image.width() as f32 * 0.15);
        for obstacle in &mut self.obstacles {
            if !obstacle.passed && bird_x > obstacle.top.x + obstacle.top.w {
                obstacle.passed = true;
                self.score += 1;
            }
        }
    }

    pub fn restart(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.state = Game::Playing;
        self.bird_pos = [50.0, 50.0];
        self.bird_velocity = 0.0;
        self.obstacles.clear();
        self.obstacle_spawn_timer = 0.0;
        self.score = 0;
        Ok(())
    }
}

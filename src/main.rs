use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Rect, Text, Font};
use ggez::input::keyboard::KeyCode;
use glam::Vec2;

struct GameState {
    player_paddle: Rect,
    ai_paddle: Rect,
    ball: Rect,
    ball_velocity: Vec2,
    player_speed: f32,
    ai_speed: f32,
    player_score: u32,
    ai_score: u32,
    lives: u32,
    game_over: bool,
    game_started: bool,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            player_paddle: Rect::new(10.0, 250.0, 10.0, 100.0),
            ai_paddle: Rect::new(780.0, 250.0, 10.0, 100.0),
            ball: Rect::new(400.0, 300.0, 10.0, 10.0),
            ball_velocity: Vec2::new(5.0, 5.0),
            player_speed: 10.0,
            ai_speed: 5.0,
            player_score: 0,
            ai_score: 0,
            lives: 5,
            game_over: false,
            game_started: false,
        }
    }

    fn reset_ball (&mut self) {
        self.ball = Rect::new(400.0, 300.0, 10.0, 10.0);
        self.ball_velocity = Vec2::new(-self.ball_velocity.x, self.ball_velocity.y);
    }

    fn reset_game(&mut self) {
        self.reset_ball();
        self.player_paddle.y = 250.0;
        self.ai_paddle.y = 250.0;
        self.player_score = 0;
        self.ai_score = 0;
        self.lives = 5;
        self.game_over = false;
        self.game_started = true;
    }
}

impl EventHandler for GameState {
    
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.game_started || self.game_over{
            return Ok(());
        }

        self.ball.x += self.ball_velocity.x;
        self.ball.y += self.ball_velocity.y;

        //Ball collision with top and bottom
        if self.ball.y <= 0.0 || self.ball.y + self.ball.h  >= 600.0 {
            self.ball_velocity.y = -self.ball_velocity.y;
        }

        //Ball collision with paddles
        if self.ball.overlaps(&self.player_paddle) || self.ball.overlaps(&self.ai_paddle) {
            self.ball_velocity.x = -self.ball_velocity.x;
        }

        // Score update and game reset
        if self.ball.x <= 0.0 {
            self.ai_score += 1;
            self.lives -= 1;
            self.reset_ball();
        } else if self.ball.x + self.ball.w >= 800.0 {
            self.player_score += 1;
            self.reset_ball();
        }

        //Check for game over
        if self.lives == 0 {
            self.game_over = true;
        }

        //Ai paddle movement
        if self.ai_paddle.y + self.ai_paddle.h / 2.0 < self.ball.y {
            self.ai_paddle.y += self.ai_speed;
        } else if self.ai_paddle.y + self.ai_paddle.h /2.0 > self.ball.y {
            self.ai_paddle.y -= self.ai_speed;
        }

        let distance_to_ball = (self.ai_paddle.y + self.ai_paddle.h / 2.0 - self.ball.y).abs();
        self.ai_speed = 5.0 + (600.0 - distance_to_ball) / 100.0;

        self.player_paddle.y = self.player_paddle.y.clamp(0.0, 500.0);
        self.ai_paddle.y = self.ai_paddle.y.clamp(0.0, 500.0);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        if self.game_over {
            let game_over_text = Text::new(("Game Over", Font::default(), 50.0));
            graphics::draw(ctx, &game_over_text, (ggez::mint::Point2 {x: 300.0, y: 250.0}, Color::BLACK))?;

            let restart_text = Text::new(("Press Space to Restart", Font::default(), 40.0));
            graphics::draw(ctx, &restart_text, (ggez::mint::Point2 {x: 270.0, y: 300.0}, Color::BLACK))?;
        } else if !self.game_started {
            let start_text = Text::new(("Press Space to Start", Font::default(), 50.0));
            graphics::draw(ctx, &start_text, (ggez::mint::Point2 {x: 250.0, y: 250.0}, Color::BLACK))?;
        } else {
            let player_paddle_mesh = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(), 
                self.player_paddle, 
                Color::BLUE,
            )?;

            let ai_paddle_mesh = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(), 
                self.ai_paddle, 
                Color::RED,
            )?;

            let ball_mesh = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(), 
                self.ball, 
                Color::BLACK,
            )?;

            let player_score_text = Text::new((format!("Player: {}", self.player_score), Font::default(), 24.0));
            let ai_score_text = Text::new((format!("AI: {}", self.ai_score), Font::default(), 24.0));
            let lives_text = Text::new((format!("Lives: {}", self.lives), Font::default(), 24.0));

            graphics::draw(ctx, &player_paddle_mesh, DrawParam::default())?;
            graphics::draw(ctx, &ai_paddle_mesh, DrawParam::default())?;
            graphics::draw(ctx, &ball_mesh, DrawParam::default())?;
            graphics::draw(ctx, &player_score_text, (ggez::mint::Point2 {x: 20.0, y: 20.0}, Color::BLACK))?;
            graphics::draw(ctx, &ai_score_text, (ggez::mint::Point2 {x: 700.0, y: 20.0}, Color::BLACK))?;
            graphics::draw(ctx, &lives_text, (ggez::mint::Point2 {x: 350.0, y: 20.0}, Color::BLACK))?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            keycode: KeyCode,
            _keymods: ggez::input::keyboard::KeyMods,
            _repeat: bool,
        ) {
        
        
        match keycode {
            KeyCode::Space => {
                if self.game_over {
                    self.reset_game();
                } else if !self.game_started {
                    self.game_started = true;
                }
            }
            KeyCode::Up => {
                if self.game_started {
                    self.player_paddle.y -= self.player_speed;
                }
            }
            KeyCode::Down => {
                if self.game_started {
                    self.player_paddle.y += self.player_speed;
                }
            }
            _ => (),
        }
    }
}

fn main() -> GameResult<()> {
    let cb = ggez::ContextBuilder::new("pong", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));

    let (ctx, event_loop) = cb.build()?;
    let state = GameState::new();

    event::run(ctx, event_loop, state)
}

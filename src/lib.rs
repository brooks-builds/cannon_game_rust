use bbggez::{
    ggez::{
        event::EventHandler,
        graphics,
        graphics::{draw, drawable_size, DrawParam},
        input::mouse,
        nalgebra::{Point2, Rotation2, Vector2},
        Context, GameResult,
    },
    rand,
    rand::prelude::*,
};
use std::f32::consts::PI;

mod cannon;
mod cannonball;
mod target;
mod wind_indicator;

use cannon::Cannon;
use cannonball::CannonBall;
use target::Target;
use wind_indicator::WindIndicator;

pub struct Game {
    cannon: Cannon,
    target: Target,
    cannonball: CannonBall,
    is_firing: bool,
    gravity: Vector2<f32>,
    wind: Vector2<f32>,
    wind_indicator: WindIndicator,
    ready_to_fire: bool,
    score: u8,
}

impl Game {
    pub fn new() -> GameResult<Game> {
        let cannon = Cannon::new(0.0, 250.0 - 25.0, 100.0, 50.0);
        let target = Target::new(1490.0, 0.0, 5.0, 75.0);
        let cannonball = CannonBall::new(cannon.location_vector(), 5.0);
        let is_firing = false;
        let gravity = Vector2::new(0.0, 0.0001);
        let wind = Game::reset_wind()?;
        let wind_indicator = WindIndicator::new();
        let ready_to_fire = true;
        let score = 0;

        Ok(Game {
            cannon,
            target,
            cannonball,
            is_firing,
            gravity,
            wind,
            wind_indicator,
            ready_to_fire,
            score,
        })
    }

    fn get_mouse_location(&self, context: &mut Context) -> Vector2<f32> {
        let mouse_location = mouse::position(context);

        Vector2::new(mouse_location.x, mouse_location.y)
    }

    fn get_vector_angle(&self, vector_1: Vector2<f32>, vector_2: Vector2<f32>) -> GameResult<f32> {
        let direction = vector_1 - vector_2;

        Ok(self.get_angle(direction)?)
    }

    fn get_angle(&self, vector: Vector2<f32>) -> GameResult<f32> {
        Ok(vector.y.atan2(vector.x))
    }

    fn reset_cannonball(&mut self) -> GameResult<()> {
        self.cannonball
            .reset_location(self.cannon.location_vector())?;
        self.is_firing = false;
        self.ready_to_fire = true;

        Ok(())
    }

    fn did_miss(&self, arena_width: f32, arena_height: f32) -> GameResult<bool> {
        Ok(
            self.cannonball.location().y - self.cannonball.get_size()? > arena_height
                || self.cannonball.location().x - self.cannonball.get_size()? > arena_width,
        )
    }

    fn did_hit_target(&self) -> GameResult<bool> {
        // if top of ball is above bottom of target
        // and bottom of ball is below top of target
        // and length between center of ball and left edge or
        //   right edge is less than ball radius
        let ball_top = self.cannonball.location().y - self.cannonball.get_size()?;
        let ball_bottom = self.cannonball.location().y + self.cannonball.get_size()?;
        let target_top = self.target.location().y;
        let target_bottom = self.target.location().y + self.target.get_height()?;

        if ball_top < target_bottom && ball_bottom > target_top {
            let ball_right = self.cannonball.location().x + self.cannonball.get_size()?;
            let ball_left = self.cannonball.location().x - self.cannonball.get_size()?;
            let target_left = self.target.location().x;
            let target_right = self.target.location().x + self.target.get_width()?;

            return Ok(ball_right > target_left && ball_left < target_right);
        }

        Ok(false)
    }

    fn reset_wind() -> GameResult<Vector2<f32>> {
        let mut rng = rand::thread_rng();

        Ok(Vector2::new(
            rng.gen_range(-0.0001, 0.0001),
            rng.gen_range(-0.0001, 0.0001),
        ))
    }

    fn target_off_screen(&self, arena_height: f32) -> GameResult<bool> {
        Ok(self.target.location().y < 0.0
            || self.target.location().y + self.target.get_height()? > arena_height)
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let (arena_width, arena_height) = drawable_size(context);
        let cannon_angle = self.get_vector_angle(
            self.get_mouse_location(context),
            self.cannon.location_vector(),
        )?;

        if mouse::button_pressed(context, mouse::MouseButton::Left) && self.ready_to_fire {
            self.is_firing = true;
            self.ready_to_fire = false;
            let direction =
                (self.get_mouse_location(context) - self.cannon.location_vector()) * 0.001;
            self.cannonball.set_velocity(direction);
        }

        if self.is_firing && self.did_miss(arena_width, arena_height)? {
            self.reset_cannonball()?;
        }
        if !self.ready_to_fire {
            self.cannonball.apply_force(self.gravity);
            self.cannonball.apply_force(self.wind);
            self.cannonball.update();
        }

        if self.is_firing && self.did_hit_target()? {
            self.reset_cannonball()?;
            self.wind = Self::reset_wind()?;
            self.target.increase_speed()?;
            self.score += 1;
        }

        if self.target_off_screen(arena_height)? {
            self.target.bounce()?;
        }

        self.cannon.set_rotation(cannon_angle)?;
        self.target.move_target()?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::WHITE);

        let (arena_width, arena_height) = drawable_size(context);

        let cannon = self.cannon.draw(context)?;
        let target = self.target.draw(context)?;
        let cannonball = self.cannonball.draw(context)?;
        let wind_indicator = self.wind_indicator.draw(context)?;
        let score = graphics::Text::new(format!("Score: {}", self.score));

        draw(
            context,
            &cannonball,
            DrawParam::default().dest(self.cannonball.location()),
        )?;
        draw(
            context,
            &cannon,
            DrawParam::default()
                .rotation(self.cannon.get_rotation()?)
                .dest(self.cannon.location_point()),
        )?;

        draw(
            context,
            &target,
            DrawParam::default().dest(self.target.location()),
        )?;

        draw(
            context,
            &wind_indicator,
            DrawParam::default()
                .dest(Point2::new(arena_width / 2.0, arena_height - 20.0))
                .rotation(self.get_angle(self.wind)?),
        )?;

        draw(
            context,
            &score,
            DrawParam::default()
                .color(graphics::BLACK)
                .scale(Vector2::new(2.0, 2.0))
                .dest(Point2::new(5.0, 5.0)),
        )?;

        graphics::present(context)
    }
}

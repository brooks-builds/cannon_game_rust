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
}

impl Game {
    pub fn new() -> GameResult<Game> {
        let mut rng = rand::thread_rng();
        let cannon = Cannon::new(100.0, 100.0 - 25.0, 100.0, 50.0);
        let target = Target::new(1490.0, 100.0, 5.0, 75.0);
        let cannonball = CannonBall::new(cannon.location_vector(), 5.0);
        let is_firing = false;
        let gravity = Vector2::new(0.0, 0.0001);
        let wind = Vector2::new(
            rng.gen_range(-0.00001, 0.00001),
            rng.gen_range(-0.00001, 0.00001),
        );
        let wind_indicator = WindIndicator::new();

        Ok(Game {
            cannon,
            target,
            cannonball,
            is_firing,
            gravity,
            wind,
            wind_indicator,
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
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let cannon_angle = self.get_vector_angle(
            self.get_mouse_location(context),
            self.cannon.location_vector(),
        )?;

        if mouse::button_pressed(context, mouse::MouseButton::Left) && !self.is_firing {
            self.is_firing = true;
            let direction =
                (self.get_mouse_location(context) - self.cannon.location_vector()) * 0.001;
            self.cannonball.set_velocity(direction);
        }

        if self.is_firing {
            self.cannonball.apply_force(self.gravity);
            self.cannonball.apply_force(self.wind);
        }

        self.cannon.set_rotation(cannon_angle)?;
        self.cannonball.update();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::WHITE);

        let (arena_width, arena_height) = drawable_size(context);

        let cannon = self.cannon.draw(context)?;
        let target = self.target.draw(context)?;
        let cannonball = self.cannonball.draw(context)?;
        let wind_indicator = self.wind_indicator.draw(context)?;

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

        graphics::present(context)
    }
}

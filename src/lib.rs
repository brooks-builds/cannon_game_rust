use bbggez::ggez::{
    event::EventHandler,
    graphics,
    graphics::{draw, drawable_size, DrawParam},
    input::mouse,
    nalgebra::{Rotation2, Vector2},
    Context, GameResult,
};
use std::f32::consts::PI;

mod cannon;
mod cannonball;
mod target;

use cannon::Cannon;
use cannonball::CannonBall;
use target::Target;

pub struct Game {
    cannon: Cannon,
    target: Target,
    cannonball: CannonBall,
}

impl Game {
    pub fn new() -> Game {
        let cannon = Cannon::new(0.0, 255.0 - 25.0, 100.0, 50.0);
        let target = Target::new(1490.0, 100.0, 5.0, 75.0);
        let cannonball = CannonBall::new(100.0, 50.0, 15.0);

        Game {
            cannon,
            target,
            cannonball,
        }
    }

    fn get_mouse_location(&self, context: &mut Context) -> Vector2<f32> {
        let mouse_location = mouse::position(context);

        Vector2::new(mouse_location.x, mouse_location.y)
    }

    fn get_vector_angle(&self, vector_1: Vector2<f32>, vector_2: Vector2<f32>) -> GameResult<f32> {
        let direction = vector_1 - vector_2;

        Ok(direction.y.atan2(direction.x))
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let cannon_angle = self.get_vector_angle(
            self.get_mouse_location(context),
            self.cannon.location_vector(),
        )?;

        self.cannon.set_rotation(cannon_angle)?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::WHITE);

        let cannon = self.cannon.draw(context)?;
        let target = self.target.draw(context)?;
        let cannonball = self.cannonball.draw(context)?;

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
            &cannonball,
            DrawParam::default().dest(self.cannonball.location()),
        )?;

        graphics::present(context)
    }
}

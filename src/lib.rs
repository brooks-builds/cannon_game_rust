use bbggez::ggez::{
    event::EventHandler,
    graphics,
    graphics::{draw, DrawParam},
    Context, GameResult,
};

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
        let cannon = Cannon::new(0.0, 250.0 - 25.0, 100.0, 50.0);
        let target = Target::new(1490.0, 100.0, 5.0, 75.0);
        let cannonball = CannonBall::new(100.0, 50.0, 15.0);

        Game {
            cannon,
            target,
            cannonball,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let current_rotation = self.cannon.get_rotation()?;

        self.cannon.set_rotation(current_rotation + 0.01)?;
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
                .dest(self.cannon.location()),
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

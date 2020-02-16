use bbggez::{
    color::random_dark_color,
    ggez::{
        graphics::{Color, Mesh},
        nalgebra::{Point2, Vector2},
        Context, GameResult,
    },
    mesh::create_rect,
};

pub struct Target {
    location: Vector2<f32>,
    width: f32,
    height: f32,
    color: Color,
    speed: Vector2<f32>,
}

impl Target {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Target {
        let location = Vector2::new(x, y);
        let color = random_dark_color();
        let speed = Vector2::new(0.0, 0.01);

        Target {
            location,
            width,
            height,
            color,
            speed,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<Mesh> {
        Ok(create_rect(
            0.0,
            0.0,
            self.width,
            self.height,
            self.color,
            context,
        ))
    }

    pub fn location(&self) -> Point2<f32> {
        Point2::from(self.location)
    }

    pub fn get_height(&self) -> GameResult<f32> {
        Ok(self.height)
    }

    pub fn get_width(&self) -> GameResult<f32> {
        Ok(self.width)
    }

    pub fn move_target(&mut self) -> GameResult<()> {
        self.location += self.speed;

        Ok(())
    }

    pub fn bounce(&mut self) -> GameResult<()> {
        self.speed *= -1.0;

        Ok(())
    }

    pub fn increase_speed(&mut self) -> GameResult<()> {
        self.speed.y += 0.01;

        Ok(())
    }
}

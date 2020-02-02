use bbggez::{
    color::random_dark_color,
    ggez::{
        graphics::{Color, Mesh},
        nalgebra::{Point2, Vector2},
        Context, GameResult,
    },
    mesh::create_circle,
};

pub struct CannonBall {
    location: Vector2<f32>,
    color: Color,
    size: f32,
}

impl CannonBall {
    pub fn new(x: f32, y: f32, size: f32) -> CannonBall {
        let location = Vector2::new(x, y);
        let color = random_dark_color();

        CannonBall {
            location,
            color,
            size,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<Mesh> {
        Ok(create_circle(0.0, 0.0, self.size, self.color, context))
    }

    pub fn location(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }
}

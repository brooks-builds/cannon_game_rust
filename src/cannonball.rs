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
    velocity: Vector2<f32>,
}

impl CannonBall {
    pub fn new(location: Vector2<f32>, size: f32) -> CannonBall {
        let color = random_dark_color();
        let velocity = Vector2::new(0.0, 0.0);

        CannonBall {
            location,
            color,
            size,
            velocity,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<Mesh> {
        Ok(create_circle(0.0, 0.0, self.size, self.color, context))
    }

    pub fn location(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }

    pub fn update(&mut self) {
        self.location += self.velocity;
    }

    pub fn set_velocity(&mut self, new_velocity: Vector2<f32>) {
        self.velocity = new_velocity;
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.velocity += force;
    }
}

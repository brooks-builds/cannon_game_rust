use bbggez::{
    color::random_dark_color,
    ggez::{
        graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
        nalgebra::{Point2, Vector2},
        Context, GameResult,
    },
};

pub struct Cannon {
    location: Vector2<f32>,
    color: Color,
    width: f32,
    height: f32,
    rotation: f32,
}

impl Cannon {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Cannon {
        let location = Vector2::new(x, y);
        let color = random_dark_color();
        let rotation = 0.0;
        Cannon {
            location,
            color,
            width,
            height,
            rotation,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(
                    -(self.width / 2.0),
                    -(self.height / 2.0),
                    self.width,
                    self.height,
                ),
                self.color,
            )
            .build(context)
    }

    pub fn location_point(&self) -> Point2<f32> {
        Point2::new(self.location.x, self.location.y)
    }
    pub fn location_vector(&self) -> Vector2<f32> {
        self.location
    }
    pub fn location_vector_center(&self) -> GameResult<Vector2<f32>> {
        Ok(Vector2::new(
            self.location.x + self.width / 2.0,
            self.location.y + self.height / 2.0,
        ))
    }
    pub fn get_rotation(&self) -> GameResult<f32> {
        Ok(self.rotation)
    }

    pub fn set_rotation(&mut self, rotation: f32) -> GameResult<()> {
        self.rotation = rotation;
        Ok(())
    }
}

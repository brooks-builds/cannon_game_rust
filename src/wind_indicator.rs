use bbggez::ggez::{
    graphics,
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

pub struct WindIndicator {
    width: f32,
    height: f32,
    color: Color,
    body: Rect,
    arrows: [Point2<f32>; 3],
}

impl WindIndicator {
    pub fn new() -> WindIndicator {
        let width = 4.0;
        let height = 15.0;
        let color = graphics::BLACK;
        let body = Rect::new(0.0 - width / 2.0, 0.0 - height / 2.0, width, height);
        let arrows = [
            Point2::new(0.0 - width / 2.0, 0.0 - height / 2.0),
            Point2::new(0.0, 0.0 - height / 2.0 - 10.0),
            Point2::new(0.0 + width / 2.0, 0.0 + height / 2.0),
        ];

        WindIndicator {
            width,
            height,
            color,
            body,
            arrows,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<Mesh> {
        MeshBuilder::new()
            .rectangle(DrawMode::fill(), self.body, self.color)
            .polyline(DrawMode::fill(), &self.arrows, self.color)?
            .build(context)
    }
}

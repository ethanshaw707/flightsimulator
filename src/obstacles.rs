use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder};
use ggez::{Context, GameResult};

pub struct Obstacle {
    pub x: f32,
    pub y: f32,
    pub kind: String,
}

impl Obstacle {
    pub fn new(x: f32, y: f32, kind: &str) -> Self
{
    Obstacle {
        x,
        y,
        kind: kind.to_string(),
    }
} 
pub fn draw(&self, ctx: &mut Context) -> GameResult<Mesh>{
    match self.kind.as_str(){
        "tree" => {
            let mut builder = MeshBuilder::new();

                // Tree trunk (rectangle)
            builder.rectangle(
                DrawMode::fill(),
                graphics::Rect::new(-5.0, 0.0, 10.0, 40.0),
                Color::from_rgb(139, 69, 19),
            );

                // Tree leaves (circle)
            builder.circle(
                DrawMode::fill(),
                [0.0, -10.0],
                25.0,
                0.1,
                Color::from_rgb(34, 139, 34),
                );

                builder.build(ctx)
            }
            "mountain" => Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[
                    [self.x, self.y],                 // Base left
                    [self.x + 100.0, self.y - 150.0], // Peak
                    [self.x + 200.0, self.y],         // Base right
                ],
                Color::from_rgb(100, 100, 100),
            ),
            _ => Err(ggez::GameError::RenderError(String::from(
                "Unknown obstacle type",
            ))),
    }
}
}
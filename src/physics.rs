use ggez::event::{self, KeyCode};
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use glam::Vec2;
use mint::Point2;

pub struct FlightSimulator {
    plane_pos: Vec2,
    plane_velocity: Vec2,
    plane_acceleration: Vec2,
    plane_angle: f32,
}

impl FlightSimulator {
    pub fn new(_ctx: &mut Context) -> Self {
        FlightSimulator {
            plane_pos: Vec2::new(1500.0, 300.0),
            plane_velocity: Vec2::new(0.0, 0.0),
            plane_acceleration: Vec2::new(0.0, 0.0),
            plane_angle: 0.0,
        }
    }
}

impl event::EventHandler<ggez::GameError> for FlightSimulator {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const ACCELERATION: f32 = 0.1;
        const MAX_VELOCITY: f32 = 5.0;

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.plane_acceleration.y = -ACCELERATION;
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.plane_acceleration.y = ACCELERATION;
        } else {
            self.plane_acceleration.y = 0.0;
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.plane_acceleration.x = -ACCELERATION;
            self.plane_angle = -0.1;
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.plane_acceleration.x = ACCELERATION;
            self.plane_angle = 0.1;
        } else {
            self.plane_acceleration.x = 0.0;
            self.plane_angle = 0.0;
        }

        // Update velocity with acceleration
        self.plane_velocity += self.plane_acceleration;

        // Clamp velocity to max values
        self.plane_velocity.x = self.plane_velocity.x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        self.plane_velocity.y = self.plane_velocity.y.clamp(-MAX_VELOCITY, MAX_VELOCITY);

        // Update position with velocity
        self.plane_pos += self.plane_velocity;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(135, 206, 235)); // Sky blue background

        // Draw the plane
        let plane = Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &{
                let mut points = vec![
                    Point2 { x: -300.0, y: 30.0 },  // Left wing (wider along y-axis)
                    Point2 { x: -20.5, y: 0.0 },    // Left body (pushed in)
                    Point2 { x: -10.0, y: -20.0 },  // Left tail (pushed in)
                ];

                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
                    points.push(Point2 { x: 0.0, y: -50.0 }); // Nose goes higher
                    points[1].y -= 10.0; // Adjust left body
                    points[2].y -= 10.0; // Adjust left tail
                } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
                    points.push(Point2 { x: 0.0, y: -10.0 }); // Nose goes lower
                    points[1].y += 10.0; // Adjust left body
                    points[2].y += 10.0; // Adjust left tail
                } else {
                    points.push(Point2 { x: 0.0, y: -30.0 }); // Default nose position
                }

                points.extend(vec![
                    Point2 { x: 10.0, y: -20.0 },   // Right tail (pushed in)
                    Point2 { x: 20.5, y: 0.0 },     // Right body (pushed in)
                    Point2 { x: 300.0, y: 30.0 },   // Right wing (wider along y-axis)
                ]);

                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
                    points[4].y -= 10.0; // Adjust right tail
                    points[5].y -= 10.0; // Adjust right body
                } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
                    points[4].y += 10.0; // Adjust right tail
                    points[5].y += 10.0; // Adjust right body
                }

                points
            },
            Color::from_rgb(255, 0, 0),
        )?;
        graphics::draw(
            ctx,
            &plane,
            DrawParam::default()
                .dest(Point2 {
                    x: self.plane_pos.x,
                    y: self.plane_pos.y,
                })
                .rotation(self.plane_angle),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}
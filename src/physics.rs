use ggez::event::{self, KeyCode};
use ggez::graphics::{self, Color, DrawParam, Mesh, PxScale, Text, TextFragment};
use ggez::{Context, GameResult};
use glam::Vec2;
use mint::Point2;

pub struct FlightSimulator {
    plane_pos: Vec2,
    plane_velocity: Vec2,
    plane_acceleration: Vec2,
    plane_angle: f32,
    crashed: bool,
    plane_image: graphics::Image,
}

impl FlightSimulator {
    pub fn new(ctx: &mut Context) -> Self {
        let plane_image = graphics::Image::new(ctx, "/b737_rear.png").expect("Failed to load plane image");

        FlightSimulator {
            plane_pos: Vec2::new(1500.0, 300.0),
            plane_velocity: Vec2::new(0.0, 0.0),
            plane_acceleration: Vec2::new(0.0, 0.0),
            plane_angle: 0.0,
            crashed: false,
            plane_image,
        }
    }

    pub fn reset(&mut self) {
        self.plane_pos = Vec2::new(1500.0, 300.0);
        self.plane_velocity = Vec2::ZERO;
        self.plane_acceleration = Vec2::ZERO;
        self.plane_angle = 0.0;
        self.crashed = false;
    }
}

impl event::EventHandler<ggez::GameError> for FlightSimulator {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.crashed {
            if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::R) {
                self.reset();
            }
            return Ok(());
        }

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

        self.plane_velocity += self.plane_acceleration;
        self.plane_velocity.x = self.plane_velocity.x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        self.plane_velocity.y = self.plane_velocity.y.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        self.plane_pos += self.plane_velocity;

        let climb_angle_degs = self.plane_velocity.y.atan2(self.plane_velocity.x).to_degrees();
        if climb_angle_degs > 45.0 && climb_angle_degs < 135.0 {
            self.plane_velocity *= 0.98;
        }

        let (width, height) = ggez::graphics::drawable_size(ctx);
        if self.plane_pos.x < 0.0
            || self.plane_pos.x > width as f32
            || self.plane_pos.y < 0.0
            || self.plane_pos.y > height as f32
        {
            self.crashed = true;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(135, 206, 235)); // Sky blue background
    
        if self.crashed {
            let crash_text = Text::new(
                TextFragment::new("You Crashed!\nPress 'R' to Restart")
                    .color(Color::from_rgb(255, 0, 0))
                    .scale(PxScale::from(48.0)),
            );
    
            let (screen_width, screen_height) = graphics::drawable_size(ctx);
            graphics::draw(
                ctx,
                &crash_text,
                DrawParam::default().dest(Point2 {
                    x: screen_width / 2.0 - 200.0,
                    y: screen_height / 2.0 - 50.0,
                }),
            )?;
        } else {
            // Draw the plane (rear view image) with scaling and rotation
            graphics::draw(
                ctx,
                &self.plane_image,
                DrawParam::default()
                    .dest(Point2 {
                        x: self.plane_pos.x,
                        y: self.plane_pos.y,
                    })
                    .rotation(self.plane_angle)
                    .offset(Point2 { x: 0.5, y: 0.5 }) // Center image on plane position
                    .scale([0.03, 0.03]), // <<=== Scale it down (adjust as needed)
            )?;
        }
    
        graphics::present(ctx)?;
        Ok(())
    }
    
}

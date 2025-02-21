#[allow(unused_imports)] 
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawParam, Image, Mesh, Text, TextFragment, PxScale};
use ggez::{Context, GameResult};
use glam::Vec2;
use mint::Point2;

pub struct FlightSimulator {
    plane_pos: Vec2,
    plane_velocity: Vec2,
    plane_acceleration: Vec2,
    plane_angle: f32,
    crashed: bool,
    thrust: f32,
    lift: f32,
    plane_image: Image,
    tunnel_x: f32,
}

impl FlightSimulator {
    pub fn new(ctx: &mut Context) -> Self {
        let plane_image = Image::new(ctx, "/b737_rear.png").expect("Failed to load plane image");

        FlightSimulator {
            plane_pos: Vec2::new(640.0, 360.0), // Start in the center of a 1280x720 screen
            plane_velocity: Vec2::ZERO,
            plane_acceleration: Vec2::ZERO,
            plane_angle: 0.0,
            crashed: false,
            thrust: 0.0,
            lift: 0.0,
            plane_image,
            tunnel_x: 500.0,
        }
    }

    pub fn reset(&mut self) {
        self.plane_pos = Vec2::new(640.0, 360.0);
        self.plane_velocity = Vec2::ZERO;
        self.plane_acceleration = Vec2::ZERO;
        self.plane_angle = 0.0;
        self.crashed = false;
        self.thrust = 0.0;
        self.lift = 0.0;
    }

    fn compute_forces(&mut self) {
        const DRAG_COEFFICIENT: f32 = 0.02;
        let drag = self.plane_velocity * DRAG_COEFFICIENT;
        self.plane_acceleration = Vec2::new(self.thrust * self.plane_angle.cos(), -self.thrust * self.plane_angle.sin()) - drag;
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

        const MAX_THRUST: f32 = 0.1; 
        const ROTATION_SPEED: f32 = 0.03; 
        const PITCH_SPEED: f32 = 0.02;  
        const TURN_SMOOTHING: f32 = 0.08; 
        const AIR_RESISTANCE: f32 = 0.01; 

        let mut yaw_change = 0.0;
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            yaw_change -= ROTATION_SPEED;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            yaw_change += ROTATION_SPEED;
        }
        self.plane_angle += yaw_change * (1.0 - TURN_SMOOTHING); 

        let mut pitch = 0.0;
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            pitch += PITCH_SPEED;  
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            pitch -= PITCH_SPEED;  
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.thrust = (self.thrust + 0.002).min(MAX_THRUST); 
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.thrust = (self.thrust - 0.002).max(0.0); 
        }

        self.compute_forces();
        let thrust_vector = Vec2::new(self.plane_angle.cos(), -self.plane_angle.sin()) * self.thrust;
        let pitch_vector = Vec2::new(0.0, pitch); 
        self.plane_velocity += thrust_vector + pitch_vector;

        self.plane_velocity *= 1.0 - AIR_RESISTANCE;
        self.plane_pos += self.plane_velocity;

        let (width, height) = ggez::graphics::drawable_size(ctx);
        if self.plane_pos.x < 0.0
            || self.plane_pos.x > width as f32
            || self.plane_pos.y < 0.0
            || self.plane_pos.y > height as f32
        {
            self.crashed = true;
        }

        self.tunnel_x -= 2.0; // Move left

// Reset tunnel position when off-screen
        if self.tunnel_x < -100.0 {
        self.tunnel_x = 1280.0;
        }  


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(135, 206, 235));

        // 1️⃣ **Sky Gradient**
        let sky_gradient = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 1280.0, 720.0),
            Color::from_rgb(100, 150, 255),
        )?;
        graphics::draw(ctx, &sky_gradient, DrawParam::default())?;

        // 2️⃣ **Clouds**
        let cloud_color = Color::new(1.0, 1.0, 1.0, 0.5);
        let cloud1 = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(300.0, 150.0, 200.0, 80.0), cloud_color)?;
        let cloud2 = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(800.0, 250.0, 250.0, 90.0), cloud_color)?;
        graphics::draw(ctx, &cloud1, DrawParam::default())?;
        graphics::draw(ctx, &cloud2, DrawParam::default())?;

        // 3️⃣ **Ground**
        let ground = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 650.0, 1280.0, 70.0),
            Color::from_rgb(50, 200, 50),
        )?;
        graphics::draw(ctx, &ground, DrawParam::default())?;

        // Mountains 
        let mountain_color1 = Color::from_rgb(139, 69, 19);
        let mountain1 = Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                Point2{x: 0.0, y: 650.0},
                Point2{x: 200.0, y: 450.0},
                Point2{x: 400.0, y: 650.0},
            ],
            mountain_color1,
        )?;
        graphics::draw(ctx, &mountain1, DrawParam::default())?;

        let mountain_color2 = Color::from_rgb(139, 69, 19);
        let mountain2 = Mesh::new_polygon(
            ctx, 
            graphics::DrawMode::fill(),
            &[
                Point2{x: 400.0, y: 650.0},
                Point2{x: 600.0, y: 500.0},
                Point2{x: 800.0, y: 650.0},
            ],
            mountain_color2
        )?;
        graphics::draw(ctx, &mountain2, DrawParam::default())?;

        // Trees
        let tree_positions = [
            Point2{x: 100.0, y: 650.0},
            Point2{x: 200.0, y: 650.0},
            Point2{x: 300.0, y: 650.0},
            Point2{x: 500.0, y: 650.0},
            Point2{x: 600.0, y: 650.0},
            Point2{x: 700.0, y: 650.0},
            Point2{x: 800.0, y: 650.0},
            Point2{x: 900.0, y: 650.0},
            Point2{x: 1000.0, y: 650.0},
            Point2{x: 1100.0, y: 650.0},
        ];

        for tree_pos in tree_positions.iter(){
        let trunk = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(tree_pos.x, tree_pos.y, 20.0, 50.0),
            Color::from_rgb(139, 69, 19),
        )?;
        graphics::draw(ctx, &trunk, DrawParam::default())?;

        let foliage = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(tree_pos.x + 10.0,tree_pos.y - 20.0, 50.0, 50.0),
            Color::from_rgb(34, 139, 34),
        )?;
        graphics::draw(ctx, &foliage, DrawParam::default())?;
        }

        let left_wall = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 10.0, 720.0),
            Color::from_rgb(100, 100, 100),
        )?;
        graphics::draw(ctx, &left_wall, DrawParam::default())?;

        let right_wall = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(1270.0, 0.0, 10.0, 720.0),
            Color::from_rgb(100, 100, 100),
        )?;
        graphics::draw(ctx, &right_wall, DrawParam::default())?;

        // Tunnel collision detection
        if (self.plane_pos.x > 300.0 && self.plane_pos.x < 350.0 && self.plane_pos.y > 200.0 && self.plane_pos.y < 600.0) ||
            (self.plane_pos.x > 800.0 && self.plane_pos.x < 850.0 && self.plane_pos.y > 200.0 && self.plane_pos.y < 600.0) {
            self.crashed = true; // Plane crashes if it hits the tunnel walls
        }



        // 4️⃣ **Draw Plane**
        if !self.crashed {
            graphics::draw(
                ctx,
                &self.plane_image,
                DrawParam::default()
                    .dest(Point2 {
                        x: self.plane_pos.x,
                        y: self.plane_pos.y,
                    })
                    .rotation(self.plane_angle)
                    .offset(Point2 { x: 0.5, y: 0.5 })
                    .scale([0.05, 0.05]), 
            )?;
        } else {
            // 5️⃣ **Crash Message**
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
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

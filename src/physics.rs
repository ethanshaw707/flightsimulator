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
        let pitch_vector = Vec2::new(self.plane_angle.sin() * pitch, -self.plane_angle.cos() * pitch);

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
        
        // tunnel
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
        let cloud_color = Color::new(1.0, 1.0, 1.0, 0.5);  //300.0 for x
        let cloud1_x = (ggez::timer::time_since_start(ctx).as_secs_f32() * 10.0) % 1280.0;
        let cloud2_x = ((ggez::timer::time_since_start(ctx).as_secs_f32() * 15.0) + 500.0) % 1280.0;
        let cloud1 = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(cloud1_x, 150.0, 200.0, 80.0), cloud_color)?;
        let cloud2 = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(cloud2_x, 250.0, 250.0, 90.0), cloud_color)?;
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

        let sun_x = (ggez::timer::time_since_start(ctx).as_secs_f32() * 50.0) % 1280.0; 
        let sun = Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Point2 { x: sun_x, y: 100.0 }, // Moves across the sky
        40.0,
        2.0,
        Color::from_rgb(255, 223, 0), // Bright yellow
        )?;
        graphics::draw(ctx, &sun, DrawParam::default())?;

        // Mountains 
        //let mountain_color1 = Color::from_rgb(139, 69, 19);

        let sun_factor = (sun_x / 1280.0).clamp(0.0, 1.0);

        let mountain_dynamic_shadow = Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &[
        Point2 { x: 100.0, y: 650.0 }, 
        Point2 { x: 250.0 + 50.0 * sun_factor, y: 500.0 },
        Point2 { x: 320.0 + 40.0 * sun_factor, y: 460.0 },
        Point2 { x: 400.0, y: 500.0 },
        Point2 { x: 550.0, y: 650.0 },
        ],
        Color::from_rgb(80, 40, 20), // Darker brown shadow
        )?;
        graphics::draw(ctx, &mountain_dynamic_shadow, DrawParam::default())?;


        let mountain1 = Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                Point2 { x: 100.0, y: 650.0 }, // Base left
                Point2 { x: 250.0, y: 500.0 }, // Mid slope
                Point2 { x: 300.0, y: 470.0 }, // Near peak (smooth transition)
                Point2 { x: 320.0, y: 460.0 }, // Peak
                Point2 { x: 340.0, y: 470.0 }, // Near peak (smooth transition)
                Point2 { x: 400.0, y: 500.0 }, // Mid slope
                Point2 { x: 550.0, y: 650.0 }, // Base right
            ],
            //mountain_color1,
            Color::from_rgb(139, 69, 19),
        )?;
        graphics::draw(ctx, &mountain1, DrawParam::default())?;

        let mountain_sunlight1 = Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                Point2 { x: 300.0, y: 470.0 },
                Point2 { x: 320.0, y: 460.0 }, // Peak
                Point2 { x: 340.0, y: 470.0 },
                Point2 { x: 400.0, y: 500.0 },
            ],
            Color::from_rgb(160, 100, 50), // Lighter brown for sunlight
        )?;
        graphics::draw(ctx, &mountain_sunlight1, DrawParam::default())?;

        let mountain_color2 = Color::from_rgb(139, 69, 19);
        let mountain2 = Mesh::new_polygon(
            ctx, 
            graphics::DrawMode::fill(),
            &[
                Point2 { x: 650.0, y: 650.0 }, // Base left
                Point2 { x: 750.0, y: 500.0 }, // Mid slope
                Point2 { x: 800.0, y: 470.0 }, // Near peak (smooth transition)
                Point2 { x: 820.0, y: 460.0 }, // Peak
                Point2 { x: 840.0, y: 470.0 }, // Near peak (smooth transition)
                Point2 { x: 900.0, y: 500.0 }, // Mid slope
                Point2 { x: 1050.0, y: 650.0 }, // Base right
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

        for tree_pos in tree_positions.iter() {
            let shadow_offset = (640.0 - sun_x) * 0.05; // Shadows move opposite the sun
            let shadow_length = (1.0 - (sun_x / 1280.0)) * 20.0; // Shadows stretch when sun is lower
        
            let tree_shadow = Mesh::new_ellipse(
                ctx,
                graphics::DrawMode::fill(),
                Point2 { x: tree_pos.x + shadow_offset, y: 680.0 }, // Moves dynamically
                30.0 + shadow_length, // Dynamic width
                8.0, 
                2.0, 
                Color::from_rgba(50, 50, 50, 150), // Darker at sunset
            )?;
            graphics::draw(ctx, &tree_shadow, DrawParam::default())?;
        
            // Tree trunk
            let trunk = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(tree_pos.x, tree_pos.y, 20.0, 50.0),
                Color::from_rgb(139, 69, 19),
            )?;
            graphics::draw(ctx, &trunk, DrawParam::default())?;
        
            // Tree foliage shading based on sun position
            let tree_light_factor = ((tree_pos.x - sun_x) / 300.0).clamp(-1.0, 1.0);
            let foliage_color = Color::from_rgb(
                (34.0 + 40.0 * tree_light_factor.abs()) as u8,
                (139.0 + 30.0 * tree_light_factor.abs()) as u8,
                34,
            );
        
            let foliage = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2 { x: tree_pos.x + 10.0, y: tree_pos.y - 20.0 },
                30.0,
                2.0,
                foliage_color,
            )?;
            graphics::draw(ctx, &foliage, DrawParam::default())?;
        }
        

    
        // Tunnel info
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
            graphics::Rect::new(0.0, 0.0, 10.0, 720.0),
            Color::from_rgb(100, 100, 100),
        )?;
        graphics::draw(ctx, &right_wall, DrawParam::default())?;

        // Tunnel collision detection
        let (width, height) = ggez::graphics::drawable_size(ctx);
let plane_size = 50.0 * 0.03; // Adjust this based on your new scale

if self.plane_pos.x < plane_size
    || self.plane_pos.x > width - plane_size
    || self.plane_pos.y < plane_size
    || self.plane_pos.y > height - plane_size
{
    self.crashed = true;
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
                    .scale([0.02, 0.02]), 
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

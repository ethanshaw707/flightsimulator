mod physics;
<<<<<<< HEAD
mod obstacles;


use ggez::{ContextBuilder, GameResult};
use ggez::conf::WindowMode;
=======
>>>>>>> 2e77ba502b3e973189e37106c460ad42ffefc29c
use physics::FlightSimulator;
use ggez::{event, ContextBuilder, GameResult};

<<<<<<< HEAD


fn main() -> GameResult {
=======
fn main() -> GameResult<()> {
>>>>>>> 2e77ba502b3e973189e37106c460ad42ffefc29c
    let (mut ctx, event_loop) = ContextBuilder::new("flight_simulator", "author_name")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0))
        .add_resource_path("resources")
        .build()
        .expect("Could not create ggez context");

    let game = FlightSimulator::new(&mut ctx);
    event::run(ctx, event_loop, game)
}

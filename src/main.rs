mod physics;
use physics::FlightSimulator;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult<()> {
    let (mut ctx, event_loop) = ContextBuilder::new("flight_simulator", "author_name")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 720.0))
        .add_resource_path("resources")
        .build()
        .expect("Could not create ggez context");

    let game = FlightSimulator::new(&mut ctx);
    event::run(ctx, event_loop, game)
}

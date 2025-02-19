mod physics;


use ggez::{ContextBuilder, GameResult};
use ggez::conf::WindowMode;
use physics::FlightSimulator;


fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("flight_simulator", "author_name")
        .window_mode(WindowMode::default().dimensions(2500.0, 1900.0)) // Set the window size to double
        .build()
        .expect("Could not create ggez context!");


    let game = FlightSimulator::new(&mut ctx);
    ggez::event::run(ctx, event_loop, game)
}

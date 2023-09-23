use engine::{
    core::Engine,
    types::{Coords, WindowSize},
};
use objects::ball::Ball;

mod engine;
mod objects;

fn main() -> Result<(), anyhow::Error> {
    let window_size = WindowSize {
        width: 800,
        height: 600,
    };
    let mut engine = Engine::new(&window_size)?;

    let ball_coords = Coords {
        x: (&window_size.width / 2) as f64,
        y: (&window_size.height / 2) as f64,
    };
    let ball = Ball::new(ball_coords, 24.0, "#cf5353");

    engine.add_game_object(ball);

    engine.run("Bouncy Ball")
}

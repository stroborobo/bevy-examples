// mod hello;
mod movement;
// use hello::HelloPlugin;
mod breakout;
use bevy::app::App;
// use breakout::BreakoutPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        // .add_plugins(HelloPlugin)
        // .add_plugins(BreakoutPlugin)
        .add_plugins(MovementPlugin)
        .run();
}

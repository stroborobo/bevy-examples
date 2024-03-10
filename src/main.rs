// mod hello;
mod movement;
// use hello::HelloPlugin;
mod breakout;

mod inspector;

use bevy::app::App;
use inspector::InspectorPlugin;
// use breakout::BreakoutPlugin;
use movement::MovementPlugin;

fn main() {
    App::new()
        // .add_plugins(HelloPlugin)
        // .add_plugins(BreakoutPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(InspectorPlugin)
        .run();
}

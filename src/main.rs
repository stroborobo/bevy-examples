use bevy::app::App;

fn main() {
    App::new()
        .add_systems(Update, hello_world)
        .run();
}

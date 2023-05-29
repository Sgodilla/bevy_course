use bevy::prelude::*;

//const BACKGROUND_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex("20232A").unwrap()))
        //.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

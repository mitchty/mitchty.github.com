use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "mitchty".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn hello_world_system() {
    // Placeholder system - replace with your actual game logic
}

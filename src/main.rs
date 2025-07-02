use bevy::prelude::*;
use bevy::window::WindowResolution;

pub mod input_gamepad;
pub mod systems;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "contro".into(),
            resolution: WindowResolution::new(1280., 720.),
            ..default()
        }),
        ..default()
    }));
    app.add_systems(
        Update,
        (systems::kill_game_on_esc, input_gamepad::gamepad_system),
    );
    app.run();
}

use bevy::prelude::*;
use bevy::window::WindowResolution;

pub mod input_gamepad;
pub mod player;
pub mod systems;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "contro".into(),
            resolution: WindowResolution::new(1920., 1080.),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }));
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (systems::kill_game_on_esc, input_gamepad::gamepad_system),
    );
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture = asset_server.load("textures/player01.png");

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(player_texture),
        Transform::from_xyz(100.0, 0.0, 0.0),
        player::Player { ..default() },
    ));
}

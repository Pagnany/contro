use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    window::{EnabledButtons, WindowResolution, WindowTheme},
};

pub mod input_gamepad;
pub mod input_keyboard;
pub mod player;
pub mod systems;

const UPDATE_INTERVAL: f64 = 1.0 / 60.0;
const WINDOW_TITLE: &str = "contro";
pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;
pub const PI: f32 = std::f32::consts::PI;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_TITLE.into(),
                name: Some(WINDOW_TITLE.into()),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    close: true,
                    maximize: false,
                    minimize: false,
                },
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }),
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 15.0,
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
            },
        },
    ));
    app.insert_resource(Time::<Fixed>::from_seconds(UPDATE_INTERVAL));
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            systems::kill_game_on_esc,
            input_gamepad::gamepad_system,
            input_keyboard::keyboard_system,
            player::player_movement_system,
        ),
    );
    // app.add_systems(FixedUpdate, player::player_movement_system);
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_texture = asset_server.load("textures/player01.png");

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(player_texture),
        Transform::from_xyz(100.0, 0.0, 0.0),
        player::Player::default(),
    ));
}

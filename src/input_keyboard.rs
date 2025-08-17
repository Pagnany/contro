use crate::{ResetSquares, player::Player};
use bevy::prelude::*;

pub fn keyboard_system(
    time: Res<Time>,
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    const KEYBOARD_ACC: f32 = 0.8;
    let mut player = player_query.single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::KeyA) {
        player.velocity.x -= KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player.velocity.x += KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        player.velocity.y += KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        player.velocity.y -= KEYBOARD_ACC * player.move_acceleration * time.delta_secs();
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        player.velocity.x += player.dash_power * player.angle.cos();
        player.velocity.y += player.dash_power * player.angle.sin();
    }
}

pub fn keyboard_reset_mysquare(
    mut square_event: EventWriter<ResetSquares>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        square_event.write(ResetSquares);
    }
}

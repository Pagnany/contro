use crate::player::Player;
use bevy::prelude::*;

pub fn gamepad_system(
    time: Res<Time>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut().unwrap();

    for (_entity, gamepad) in &gamepads {
        // if gamepad.just_pressed(GamepadButton::South) {
        //     info!("{} just pressed South", entity);
        // } else if gamepad.just_released(GamepadButton::South) {
        //     info!("{} just released South", entity);
        // }

        // From -1.0 to 1.0
        let right_trigger = gamepad.get(GamepadAxis::RightZ).unwrap() + 1.0;
        if !player.right_trigger_down && right_trigger > 0.4 {
            player.right_trigger_down = true;
            player.velocity.x += player.dash_power * player.angle.cos();
            player.velocity.y += player.dash_power * player.angle.sin();
        } else if player.right_trigger_down && right_trigger < 0.4 {
            player.right_trigger_down = false;
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_x.abs() > 0.1 || left_stick_y.abs() > 0.1 {
            // info!("LeftStick position: ({}, {})", left_stick_x, left_stick_y);
            player.velocity.x += left_stick_x * player.move_acceleration * time.delta_secs();
            player.velocity.y += left_stick_y * player.move_acceleration * time.delta_secs();
        }
    }
}

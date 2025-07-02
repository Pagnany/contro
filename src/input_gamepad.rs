use bevy::prelude::*;

pub fn gamepad_system(gamepads: Query<(Entity, &Gamepad)>) {
    for (entity, gamepad) in &gamepads {
        // if gamepad.just_pressed(GamepadButton::South) {
        //     info!("{} just pressed South", entity);
        // } else if gamepad.just_released(GamepadButton::South) {
        //     info!("{} just released South", entity);
        // }
        //
        // let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        // if right_trigger.abs() > 0.01 {
        //     info!("{} RightTrigger2 value is {}", entity, right_trigger);
        // }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_x.abs() > 0.01 || left_stick_y.abs() > 0.01 {
            info!(
                "{} LeftStick position: ({}, {})",
                entity, left_stick_x, left_stick_y
            );
        }
    }
}

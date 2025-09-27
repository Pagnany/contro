use crate::{ResetSquares, player::Player};
use bevy::{input::gamepad::GamepadEvent, prelude::*};

pub fn gamepad_system(
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut().unwrap();

    for gamepad in &gamepads {
        // The Controller I own works differently on Windows and Linux
        // Controller on Windows
        // let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        // Gamesir controller on Linux
        let right_trigger = gamepad.get(GamepadAxis::RightZ).unwrap();
        if !player.right_trigger_down && right_trigger > 0.2 {
            player.right_trigger_down = true;
            player.velocity.x += player.dash_power * player.angle.cos();
            player.velocity.y += player.dash_power * player.angle.sin();
        } else if player.right_trigger_down && right_trigger < 0.2 {
            player.right_trigger_down = false;
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_x.abs() > 0.1 || left_stick_y.abs() > 0.1 {
            player.velocity.x += left_stick_x * player.move_acceleration * time.delta_secs();
            player.velocity.y += left_stick_y * player.move_acceleration * time.delta_secs();
        }
    }
}

pub fn gamepad_braking_system(
    time: Res<Time>,
    gamepads: Query<&Gamepad>,
    mut player_query: Query<&mut Player>,
) {
    let mut player = player_query.single_mut().unwrap();

    for gamepad in &gamepads {
        // Left trigger for braking
        // Windows
        // let left_trigger = gamepad.get(GamepadButton::LeftTrigger2).unwrap();
        // Gamesir controller on Linux
        let left_trigger = gamepad.get(GamepadAxis::LeftZ).unwrap() + 1.0;

        if !player.init_left_trigger_down && left_trigger == 1.0 {
            return;
        } else {
            player.init_left_trigger_down = true;
        }

        if left_trigger > 0.2 {
            if player.velocity.x > 0.01 {
                player.velocity.x -= player.breaking_power * 1.0 * left_trigger * time.delta_secs();
                if player.velocity.x < 0.0 {
                    player.velocity.x = 0.0;
                }
            } else if player.velocity.x < -0.01 {
                player.velocity.x -=
                    player.breaking_power * -1.0 * left_trigger * time.delta_secs();
                if player.velocity.x > 0.0 {
                    player.velocity.x = 0.0;
                }
            }
            if player.velocity.y > 0.01 {
                player.velocity.y -= player.breaking_power * 1.0 * left_trigger * time.delta_secs();
                if player.velocity.y < 0.0 {
                    player.velocity.y = 0.0;
                }
            } else if player.velocity.y < -0.01 {
                player.velocity.y -=
                    player.breaking_power * -1.0 * left_trigger * time.delta_secs();
                if player.velocity.y > 0.0 {
                    player.velocity.y = 0.0;
                }
            }
        }
    }
}

pub fn gamepad_reset_mysquare(
    gamepads: Query<&Gamepad>,
    mut square_event: EventWriter<ResetSquares>,
) {
    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::South) {
            square_event.write(ResetSquares);
        }
    }
}

/// Show all gamepad input events in the log
pub fn gamepad_input_all_events(mut evr_gamepad: EventReader<GamepadEvent>) {
    for ev in evr_gamepad.read() {
        info!("Gamepad event: {:?}", ev);
    }
}

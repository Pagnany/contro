use bevy::{math::FloatPow, prelude::*};

use crate::MySquare;

#[derive(Component)]
pub struct Player {
    pub radius: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    /// Angle in radians
    pub angle: f32,
    pub move_acceleration: f32,
    pub dash_power: f32,
    pub friction: f32,
    pub right_trigger_down: bool,
    pub left_trigger_down: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            radius: 15.0,
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            angle: 0.0,
            move_acceleration: 15000.0,
            dash_power: 5000.0,
            friction: 15.0,
            right_trigger_down: false,
            left_trigger_down: false,
        }
    }
}

pub fn player_movement_system(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    let (mut player, mut transform) = query.single_mut().unwrap();

    // Update position
    // Clamp to window bounds
    transform.translation.x += player.velocity.x * time.delta_secs();
    if transform.translation.x > crate::WINDOW_WIDTH / 2.0 {
        transform.translation.x = crate::WINDOW_WIDTH / 2.0;
    } else if transform.translation.x < -crate::WINDOW_WIDTH / 2.0 {
        transform.translation.x = -crate::WINDOW_WIDTH / 2.0;
    }
    transform.translation.y += player.velocity.y * time.delta_secs();
    if transform.translation.y > crate::WINDOW_HEIGHT / 2.0 {
        transform.translation.y = crate::WINDOW_HEIGHT / 2.0;
    } else if transform.translation.y < -crate::WINDOW_HEIGHT / 2.0 {
        transform.translation.y = -crate::WINDOW_HEIGHT / 2.0;
    }

    // Rotate the player based on velocity
    if player.velocity.length() > 0.0 {
        player.angle = player.velocity.y.atan2(player.velocity.x);

        // texture is rotated
        // so PI / 2.0 is subtracted
        transform.rotation = Quat::from_rotation_z(player.angle - crate::PI / 2.0);
    }

    player.velocity.x -= player.velocity.x * player.friction * time.delta_secs();
    player.velocity.y -= player.velocity.y * player.friction * time.delta_secs();
}

pub fn player_mysquare_collision_system(
    mut commands: Commands,
    player_query: Query<(&Player, &Transform)>,
    square_query: Query<(&MySquare, &Transform, Entity)>,
) {
    let (player, player_transform) = player_query.single().unwrap();

    if player.velocity.length() > 1500.0 {
        return;
    }

    for (square, square_transform, square_entity) in square_query.iter() {
        let half_size = square.size / 2.0;
        let delta_x = player_transform.translation.x - square_transform.translation.x;
        let delta_y = player_transform.translation.y - square_transform.translation.y;
        let closest_x = delta_x.clamp(-half_size.x, half_size.x);
        let closest_y = delta_y.clamp(-half_size.y, half_size.y);
        let distance_x = delta_x - closest_x;
        let distance_y = delta_y - closest_y;
        let distance_squared = distance_x.squared() + distance_y.squared();
        if distance_squared < player.radius.squared() {
            // info!("Player collided with square: {:?}", square_entity);
            commands.entity(square_entity).despawn();
        }
    }
}

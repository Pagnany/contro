use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
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

    transform.translation.x += player.velocity.x * time.delta_secs();
    transform.translation.y += player.velocity.y * time.delta_secs();

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

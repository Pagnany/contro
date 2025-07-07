use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub move_acceleration: f32,
    pub dash_power: f32,
    pub friction: f32,
}

pub fn player_movement_system(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    let (mut player, mut transform) = query.single_mut().unwrap();
    transform.translation.x += player.velocity.x * time.delta_secs();
    transform.translation.y += player.velocity.y * time.delta_secs();

    player.velocity.x -= player.velocity.x * player.friction * time.delta_secs();
    player.velocity.y -= player.velocity.y * player.friction * time.delta_secs();
}

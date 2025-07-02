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

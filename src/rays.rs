use bevy::{prelude::*, math::f32::Vec3};

#[derive(Component)]
pub struct Ray { pub vel: Vec3 }

#[derive(Component)]
pub struct Position { pub pos: Vec3 }

#[derive(Component)]
pub struct Velocity { pub vel: Vec3 }

impl Ray {
    pub fn new(vel: Vec3) -> Ray{
        Ray { vel }
    }
}

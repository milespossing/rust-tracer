use amethyst::{
    core::math::{Vector3},
    ecs::{Component, VecStorage},
};

#[derive(Debug)]
pub struct Ray {
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
}

impl Ray {
    pub fn new(pos: Vector3<f32>, vel: Vector3<f32>) -> Ray{
        Ray { pos, vel }
    }
}

impl Component for Ray {
    type Storage = VecStorage<Self>;
}


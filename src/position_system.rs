use amethyst::{
    ecs::{System, WriteStorage, Read, Join},
    core::timing::Time,
};
use crate::rays::Ray;

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
    type SystemData = WriteStorage<'a, Ray>;

    fn run(&mut self, mut rays: Self::SystemData) {
        for ray in (&mut rays).join(){
            ray.pos = ray.pos + ray.vel;        
        }
    }
}


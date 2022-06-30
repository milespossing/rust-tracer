use bevy::prelude::*;
use crate::rays::{Velocity};

pub fn position_system(mut query: Query<(&mut Transform, &Velocity)>) {
  for (mut trans, vel) in query.iter_mut() {
    trans.translation += vel.vel / 60f32;
  }
}

pub fn logging_system(query: Query<&Transform>) {
  for ray in query.iter() {
    println!("{:?}", ray);
  }
}

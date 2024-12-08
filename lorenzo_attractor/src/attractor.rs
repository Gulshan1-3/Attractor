
use bevy::prelude::*;
pub struct Attractors {
    a:f32,
    b:f32,
    c:f32,
    t:f32,
}

pub fn lorenz_test_values() -> Attractors {
    Attractors{
        a: 10.0,
        b: 28.0,
        c: 2.66,
        t: 0.01,
    }
}

fn lorenz_solution (position:transform,constants:attractors) -> Vec3  {
    let Vec3{x,y,z} = position.translation;
    let dx =  x + constants.t * constants.a * (y-x);
    let dy =  y + constants.t * (x*(constants.b - z)- y);
    let dz  = z + constants.t (x*y-constants.c*z);
    Vec3::new(dx, dy, dz)
}

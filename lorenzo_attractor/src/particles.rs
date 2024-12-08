use crate::attractors;
use bevy::prelude::*;
use rand::prelude::random;

////////////////////////////////////////////////////
// PARTICLE CONSTANTS
///////////////////////////////////////////////////

pub const PARTICLE_SIZE: f32 = 0.2;

const N_PARTCLES: u32 = 3000;

const RANDOM_MULTIPLIER: f32 = 4.0;


pub struct Materials {
    pub particle_material: Handle<ColorMaterial>,
    pub new_particle_material: Handle<ColorMaterial>,
}

pub struct Particle {
    pub new: bool,
}


pub fn spawn_particles(mut commands: Commands, materials: Res<Materials>) {
    (0..N_PARTCLES).for_each(|_| spawn_particle(&mut commands, &materials));
}

pub fn spawn_particle(commands: &mut Commands, materials: &Res<Materials>) {
    let material = materials.particle_material.clone(); // Avoid cloning in the spawn call
    let transform = Transform::from_xyz(
        random::<f32>() * RANDOM_SPAWNER_MULTIPLIER,
        random::<f32>() * RANDOM_SPAWNER_MULTIPLIER,
        1.0,
    );

    commands.spawn_bundle(SpriteBundle {
        material,
        sprite: Sprite::new(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
        transform,
        ..Default::default()
    })
    .insert(Particle { new: false });
}
pub fn particle_movement(mut query: Query<(&Particle,&mut Transform,)>) {    
    for mut transform in query.iter_mut() {
        
        let new_position = attractors::solve_lorenz(&transform.translation, attractors::LORENZ_TEST);
        transform.translation = new_position;
    }
}
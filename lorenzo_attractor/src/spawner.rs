use crate::particles::{Materials, Particle, PARTICLE_SIZE};
use bevy::prelude::*;


pub fn spawner_system(
    mut commands: Commands,
    materials: Res<Materials>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        // List of positions where particles will be spawned
        let positions = vec![
            Vec3::new(3.0, 5.0, 1.0),
            Vec3::new(-3.0, -5.0, 1.0),
        ];

        // Spawn particles at each position
        for position in positions {
            commands.spawn_bundle(SpriteBundle {
                material: materials.new_particle_material.clone(),
                sprite: Sprite::new(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .insert(Particle { new: false });
        }
    }
}

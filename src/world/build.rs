use bevy::prelude::*;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};

use super::{CHUNK_HEIGHT, CHUNK_SIZE};

fn build_world_data() -> Vec<Vec<Vec<i32>>> {
    let mut map = vec![vec![vec![0; CHUNK_SIZE as usize]; CHUNK_HEIGHT as usize]; CHUNK_SIZE as usize];
    
    let mut noise = FastNoise::new();
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.6);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(5.0);
        

    for y in 0..CHUNK_HEIGHT {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let mod_val = noise.get_noise3d(x as f32 / 50., y as f32 / 500., z as f32 / 50.) * 5.;
                let val = noise.get_noise(x as f32 / 100. * mod_val, y as f32 / 100. * mod_val) * 2.;

                if y <= 60 + (val * 10.).floor() as i32 {
                    map[x as usize][y as usize][z as usize] = 1;   
                }
            }
        }
    }

    map
}

pub fn build_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let world_data = build_world_data();
    
    for y in 0..CHUNK_HEIGHT {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if world_data[x as usize][y as usize][z as usize] != 0 {
                    commands.spawn((
                        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
                        MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 255))),
                        Transform::from_xyz(x as f32, y as f32, z as f32)
                    ));
                }
            }
        }
    }

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5., 100., 5.)
    ));
}
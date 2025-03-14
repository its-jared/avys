use bevy::{asset::RenderAssetUsages, gizmos::light, prelude::*, render::mesh::{Indices, PlaneMeshBuilder, PrimitiveTopology, VertexAttributeValues}};
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

fn build_chunk_mesh(data: Vec<Vec<Vec<i32>>>) -> Mesh {
    let mut noise = FastNoise::new();
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.6);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(5.0);
    
    let mut mesh = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(50., 50.)
            .subdivisions(100)
    );

    if let Some(VertexAttributeValues::Float32x3(positions))
        = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            for pos in positions.iter_mut() {
                let mod_val = noise.get_noise3d(pos[0] as f32 / 50., pos[1] as f32 / 500., pos[2] as f32 / 50.) * 5.;
                let val = noise.get_noise(pos[0] as f32 / 100. * mod_val, pos[1] as f32 / 100. * mod_val) * 2.;

                pos[1] = val * 10.;
            }
        }

    mesh
}

pub fn build_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let chunk_data = build_world_data();
    let chunk_mesh = build_chunk_mesh(chunk_data);

    commands.spawn((
        Mesh3d(meshes.add(chunk_mesh)),
        MeshMaterial3d(materials.add(Color::srgb(0., 1., 1.))),
        Transform::from_xyz(0., 0., 0.),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 10., 0.)
    ));
}
use bevy::{ecs::system::SystemState, prelude::*, render::mesh::VertexAttributeValues};
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};

use crate::player::PlayerCharacter;

use super::{Chunk, ChunkStore, SpawnChunk, CHUNK_HEIGHT, CHUNK_SIZE};

pub fn build_chunk_data() -> Vec<Vec<Vec<i32>>> {
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
                let mod_val = noise.get_noise3d(x as f32 / 50., y as f32 / 50., z as f32 / 50.) * 2.;
                let val = noise.get_noise(x as f32 / 10. * mod_val, y as f32 / 10. * mod_val) * 2.;

                if y <= 60 + (val * 10.).floor() as i32 {
                    map[x as usize][y as usize][z as usize] = 1;   
                }
            }
        }
    }

    map
}

pub fn build_chunk_mesh(_data: Vec<Vec<Vec<i32>>>, offset: IVec2) -> Mesh {
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
            .size(CHUNK_SIZE as f32, CHUNK_SIZE as f32)
            .subdivisions(10)
    );

    if let Some(VertexAttributeValues::Float32x3(positions))
        = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            for pos in positions.iter_mut() {
                let mod_val = noise.get_noise3d(
                    pos[0] + (offset.x * CHUNK_SIZE as i32) as f32 / 50., 
                    pos[1] as f32 / 500., 
                    pos[2] + (offset.y * CHUNK_SIZE as i32) as f32 / 50.
                ) * 5.;
                let val = noise.get_noise(
                    pos[0] + (offset.x * CHUNK_SIZE as i32) as f32 / 100. * mod_val, 
                    pos[1] + (offset.y * CHUNK_SIZE as i32) as f32 / 100. * mod_val
                ) * 2.;

                pos[1] = val * 5.;
            }

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_, g, _]| {
                    let g = *g / 5. * 2.;

                    [g, g, g, 1.]
                }).collect();
            
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors
            );
        }

    mesh.compute_normals();

    mesh
}

pub fn manage_chunks(
    mut commands: Commands,
    mut current_chunk: Local<IVec2>,
    mut chunk_store: ResMut<ChunkStore>,
    q_player: Query<&Transform, With<PlayerCharacter>>,
    q_chunks: Query<(Entity, &Mesh3d), With<Chunk>>,
) {
    let player_trans = q_player.single();
    let xz = (player_trans.translation.xz() / Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32))
        .trunc()
        .as_ivec2();

    if *current_chunk != xz {
        *current_chunk = xz;

        let chunks_to_render = [
            *current_chunk + IVec2::new(-1, -1),
            *current_chunk + IVec2::new(-1, 0),
            *current_chunk + IVec2::new(-1, 1),
            *current_chunk + IVec2::new(0, -1),
            *current_chunk + IVec2::new(0, 0),
            *current_chunk + IVec2::new(0, 1),
            *current_chunk + IVec2::new(1, -1),
            *current_chunk + IVec2::new(1, 0),
            *current_chunk + IVec2::new(1, 1),
        ];

        let chunks_to_despawn: Vec<(IVec2, Handle<Mesh>)> =
            chunk_store
                .0
                .clone()
                .into_iter()
                .filter(|(key, _)| {
                    !chunks_to_render.contains(&key)
                })
                .collect();
        
        for (chunk, mesh) in chunks_to_despawn {
            let Some((entity, _)) = q_chunks
                .iter()
                .find(|(_, handle)| ***handle == mesh)
            else {
                continue;
            };

            commands.entity(entity).despawn_recursive();
            chunk_store.0.remove(&chunk);
        }

        for chunk in chunks_to_render {
            commands.queue(SpawnChunk(chunk));
        }
    }
}

pub fn build_world(mut commands: Commands) {
    commands.queue(SpawnChunk(IVec2::new(0, 0)));
    commands.queue(SpawnChunk(IVec2::new(1, 0)));
    commands.queue(SpawnChunk(IVec2::new(0, 1)));
    commands.queue(SpawnChunk(IVec2::new(1, 1)));
    commands.queue(SpawnChunk(IVec2::new(-1, 0)));
    commands.queue(SpawnChunk(IVec2::new(0, -1)));
    commands.queue(SpawnChunk(IVec2::new(-1, -1)));
    commands.queue(SpawnChunk(IVec2::new(1, -1)));
    commands.queue(SpawnChunk(IVec2::new(-1, 1)));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 2., 0.)
    ));
}
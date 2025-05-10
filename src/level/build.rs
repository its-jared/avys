use bevy::prelude::*;
use bracket_noise::prelude::*;
use crate::level::{biomes::biome, block::BlockRegistery};
use super::infinite::ChunkStore;

pub const CHUNK_SIZE: f32 = 16.0;
pub const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Chunk;

pub fn build_world(mut cmds: Commands) {
    cmds.queue(BuildChunk(IVec2::new(-1, -1)));
    cmds.queue(BuildChunk(IVec2::new(-1, 0)));
    cmds.queue(BuildChunk(IVec2::new(-1, 1)));
    cmds.queue(BuildChunk(IVec2::new(0, -1)));
    cmds.queue(BuildChunk(IVec2::new(0, 0)));
    cmds.queue(BuildChunk(IVec2::new(0, 1)));
    cmds.queue(BuildChunk(IVec2::new(1, -1)));
    cmds.queue(BuildChunk(IVec2::new(1, 0)));
    cmds.queue(BuildChunk(IVec2::new(1, 1)));
}

pub struct BuildChunk(pub IVec2);

impl Command for BuildChunk {
    fn apply(self, world: &mut World) -> () {
        if world
            .get_resource_mut::<ChunkStore>()
            .expect("ChunkStore to be available")
            .0
            .get(&self.0)
            .is_some()
        {
            warn!("Chunk {} already exists", self.0);
            return;
        };

        info!("Building chunk at {}", self.0);

        let block_registery = &world
            .get_resource_mut::<BlockRegistery>()
            .expect("Waiting for Block Registery")
            .blocks
            .clone();
        let asset_server = world.get_resource::<AssetServer>().unwrap().clone();
        let biomes = biome::set_registery();

        let chunk_entity = world.spawn((
            Transform::from_xyz(
                (self.0.x as f32 * TILE_SIZE) * CHUNK_SIZE, 
                (self.0.y as f32 * TILE_SIZE) * CHUNK_SIZE, 
                0.0
            ),
            Chunk,
        )).id();

        let mut noise = FastNoise::seeded(10);
        noise.set_noise_type(NoiseType::Cubic);
        noise.set_frequency(0.005);

        for y in 0..50 {
            for x in 0..50 {
                let nb = noise.get_noise(
                    (x as f32 + (self.0.x as f32 + CHUNK_SIZE)) * TILE_SIZE,
                    (y as f32 + (self.0.y as f32 + CHUNK_SIZE)) * TILE_SIZE,
                );
                let mut biome = &biomes.biomes[0];

                if nb >= 0.0 { biome = &biomes.biomes[1]; }

                let block = biome.get_block(IVec2 {
                    x: ((x as f32 + (self.0.x as f32 + CHUNK_SIZE)) * TILE_SIZE) as i32,
                    y: ((y as f32 + (self.0.y as f32 + CHUNK_SIZE)) * TILE_SIZE) as i32,
                });

                let tile = world.spawn((
                    Sprite {
                        image: asset_server.load(
                            format!("textures/{}.png", block_registery[block].texture),
                        ),
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    Transform::from_xyz(
                        (x as f32 + (self.0.x as f32 + CHUNK_SIZE)) * TILE_SIZE, 
                        (y as f32 + (self.0.y as f32 + CHUNK_SIZE)) * TILE_SIZE, 
                        0.0
                    ),
                )).id();

                world.entity_mut(chunk_entity).add_child(tile);
            }
        }
    }
}

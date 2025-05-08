use std::collections::HashMap;
use bevy::prelude::*;
use bracket_noise::prelude::*;
use crate::level::biome;

use super::{biome::BiomeRegistery, infinite::ChunkStore};

pub const CHUNK_SIZE: f32 = 16.0;
pub const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Chunk;

fn build_world(mut cmds: Commands) {
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

        let chunk_entity = world.spawn((
            Transform::from_xyz(
                (self.0.x as f32 * TILE_SIZE) * CHUNK_SIZE, 
                (self.0.y as f32 * TILE_SIZE) * CHUNK_SIZE, 
                0.0
            ),
            Chunk,
        )).id();
    
        let biome_registery = *world
            .get_resource::<BiomeRegistery>()
            .expect("Waiting for Biome Registery");
    
        for y in 0..50 {
            for x in 0..50 {
                let biome = biome::get_biome(
                    IVec2::new(
                        ((x as f32 + (self.0.x as f32 + CHUNK_SIZE)) * TILE_SIZE) as i32,
                        ((y as f32 + (self.0.y as f32 + CHUNK_SIZE)) * TILE_SIZE) as i32,
                    ),
                    biome_registery.0
                );

                let tile = world.spawn((
                    Sprite::from_color(Color::srgb(biome.ground_color.0, biome.ground_color.1, biome.ground_color.2), Vec2::new(TILE_SIZE, TILE_SIZE)),
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

pub struct LevelBuilderPlugin;

impl Plugin for LevelBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChunkStore(HashMap::new()))

            .add_systems(
                Startup, 
                build_world
                    .run_if(resource_exists::<ChunkStore>)
                    .run_if(resource_exists::<BiomeRegistery>)
            );
            /*.add_systems(
                Update,
                super::infinite::manage_chunks
                    .run_if(resource_exists::<ChunkStore>)
            );*/
    }
}
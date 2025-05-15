use std::collections::HashMap;
use bevy::prelude::*;
use rand::Rng;
use crate::level::{biomes::biome, block::BlockRegistery};

pub const CHUNK_SIZE: f32 = 32.0;
pub const TILE_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Chunk(pub HashMap<IVec2, usize>);

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
        info!("Building chunk at: {}", self.0);

        let asset_server = world.get_resource::<AssetServer>().unwrap().clone();
        let mut rand = rand::rng();

        let biome_registery = &biome::get_registery();
        let block_registery = &world.get_resource::<BlockRegistery>().unwrap().clone();
        
        let chunk_entity = world.spawn(
            Transform::from_xyz(
                (self.0.x as f32 * TILE_SIZE) * CHUNK_SIZE, 
                (self.0.y as f32 * TILE_SIZE) * CHUNK_SIZE, 
                0.0
            )
        ).id();

        for y in 0..CHUNK_SIZE as i32 + 1 {
            for x in 0..CHUNK_SIZE as i32 + 1 {
                let pos = IVec2::new(
                    ((x as f32 + (self.0.x as f32 + CHUNK_SIZE)) * TILE_SIZE) as i32, 
                    ((y as f32 + (self.0.y as f32 + CHUNK_SIZE)) * TILE_SIZE) as i32,
                );
                let biome = biome::get_biome(pos, biome_registery);
                let ground_block = block_registery.blocks.get(&biome.get_ground(pos)).unwrap().clone();
                let decor_block = block_registery.blocks.get(&biome.get_decor(pos)).unwrap().clone();

                let mut ground_texture_path = format!("textures/{}.png", ground_block.texture);
                if ground_block.multiple_textures {
                    let index = rand.random_range(0..ground_block.textures.len());
                    ground_texture_path = format!("textures/{}/{}.png",
                        ground_block.texture,
                        ground_block.textures.get(index).unwrap().clone());
                }

                let mut decor_texture_path = format!("textures/{}.png", decor_block.texture);
                if decor_block.multiple_textures {
                    let index = rand.random_range(0..decor_block.textures.len());
                    decor_texture_path = format!("textures/{}/{}.png",
                        decor_block.texture,
                        decor_block.textures.get(index).unwrap().clone());
                }

                let ground_tile = world.spawn((
                    Sprite {
                            image: asset_server.load(ground_texture_path),
                            custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                            ..default()
                        },
                    Transform {
                        translation: Vec3::new(
                            (x as f32 + (self.0.x as f32 + CHUNK_SIZE)) * TILE_SIZE, 
                            (y as f32 + (self.0.y as f32 + CHUNK_SIZE)) * TILE_SIZE, 
                            0.0
                        ),
                        ..default()
                    }
                )).id();
                let decor_tile = world.spawn((
                    Sprite {
                            image: asset_server.load(decor_texture_path),
                            custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                            ..default()
                        },
                    Transform {
                        translation: Vec3::new(
                            (x as f32 + (self.0.x as f32 + CHUNK_SIZE)) * TILE_SIZE, 
                            (y as f32 + (self.0.y as f32 + CHUNK_SIZE)) * TILE_SIZE, 
                            1.0
                        ),
                        ..default()
                    }
                )).id();

                world.entity_mut(chunk_entity)
                    .add_child(ground_tile)
                    .add_child(decor_tile);
            }
        }
    }
}

#[allow(dead_code)]
fn get_random_block_rotation(rnd: &mut rand::rngs::ThreadRng) -> Quat {
    let rand_face = rnd.random_range(0..3);

    if rand_face == 0 {
        return Quat::from_rotation_z(0.0);
    }
    else if rand_face == 1 {
        return Quat::from_rotation_z(f32::to_radians(90.0));
    }
    else if rand_face == 2 {
        return Quat::from_rotation_z(f32::to_radians(-90.0));
    }
    else if rand_face == 3 {
        return Quat::from_rotation_z(f32::to_radians(180.0));
    }

    Quat::from_rotation_z(0.0)
}
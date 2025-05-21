use std::collections::HashMap;
use bevy::prelude::*;
use block::Block;

pub mod build;
pub mod block;

pub const TILE_SIZE: i32 = 64;
pub const LEVEL_SIZE: i32 = 100;

pub fn world_to_level_pos(wpos: Vec2) -> IVec2 {
    IVec2 {
        x: (wpos.x) as i32 / TILE_SIZE,
        y: (wpos.y) as i32 / TILE_SIZE,
    }
}

pub fn level_to_world_pos(lpos: IVec2, z: f32) -> Vec3 {
    Vec3 { 
        x: (lpos.x * TILE_SIZE) as f32, 
        y: (lpos.y * TILE_SIZE) as f32,
        z
    }
}

#[derive(Resource)]
pub struct Level {
    pub blocks: HashMap<IVec2, usize>,
    pub block_entities: HashMap<IVec2, Entity>,
    pub block_registery: Vec<Block>,
}

#[allow(dead_code)]
impl Level {
    fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            block_entities: HashMap::new(),
            block_registery: vec!(
                Block::new(0, String::from("textures/air.png")),
                Block::new(1, String::from("textures/basalt_wall.png")),
                Block::new(2, String::from("textures/moss.png")),
                Block::new(3, String::from("textures/basalt.png")),
            )
        }
    }

    pub fn set_block(
        &mut self, c: &mut Commands, a: &AssetServer,
        pos: IVec2, id: usize
    ) {
        if let Some(block) = self.block_registery.get(id) {
            self.blocks.insert(pos, id);
            
            let block_entity = c.spawn((
                Transform::from_translation(level_to_world_pos(pos, 0.0)),
                Sprite {
                    image: a.load(block.texture_location.clone()),
                    custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..default()
                }
            )).id();

            self.block_entities.insert(pos, block_entity);
        }
    }

    pub fn remove_block(
        &mut self, c: &mut Commands,
        pos: IVec2
    ) -> bool {
        if self.blocks.contains_key(&pos)
            && self.block_entities.contains_key(&pos) 
        {
            self.blocks.remove(&pos);

            let entity = self.block_entities
                .get(&pos)
                .unwrap()
                .to_owned();
            c.entity(entity).despawn();
            self.block_entities.remove(&pos);

            return true;
        }
        
        false
    }

    pub fn print_blocks(&self) {
        for block in &self.blocks {
            println!("pos: {}, id: {}", block.0, block.1);
        }

        println!("===")
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Level::new())
            .add_systems(Startup, build::build_level);
    }
}
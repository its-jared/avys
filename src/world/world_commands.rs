use bevy::ecs::{system::Commands, world::{Command, World}};
use bevy::prelude::*;
use crate::data::BlockResource;

use super::*;

pub struct SetBlock {
    pub id: usize,
    pub pos: Vec3,
}

impl Command for SetBlock {
    fn apply(self, world: &mut World) {
        let block_resource: Option<&BlockResource> = world.get_resource();
        let asset_server: Option<&AssetServer> = world.get_resource();
        let global_pos = world_to_global_pos(self.pos);
        let block = block_resource.unwrap().0.get(self.id)
            .expect(format!("Block `{}` not found!", self.id).as_str()); 
        
        world.spawn((
            Transform::from_translation(Vec3::new(
                global_pos.x,
                global_pos.y,
                0.0
            )),
            Sprite {
                image: asset_server.unwrap().load(block.img_path.as_str()),
                color: Color::srgba(1.0, 1.0, 1.0, block.alpha),
                custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                ..Default::default()
            },
        ));
    }
}

pub trait WorldCommands {
    fn set_block(&mut self, id: usize, pos: Vec3);
}

impl<'w, 's> WorldCommands for Commands<'w, 's> { 
    fn set_block(&mut self, id: usize, pos: Vec3) {
        self.queue(SetBlock {
            id, 
            pos
        });
    }
}
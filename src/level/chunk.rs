use std::collections::HashMap;
use bevy::prelude::*;
use super::*;

#[derive(Clone)]
pub struct Chunk {
    pub floor_layer: HashMap<IVec2, (usize, Entity)>,
    pub wall_layer: HashMap<IVec2, (usize, Entity)>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            floor_layer: HashMap::new(),
            wall_layer: HashMap::new()
        }
    }

    pub fn set_block(
    &mut self, c: &mut Commands, a: &AssetServer, level: &Level,
    pos: IVec2, id: usize
    ) {
        if let Some(block) = level.block_registery.get(id) {
            let block_entity: Entity;

            if !block.solid {
                block_entity = c.spawn((
                    Transform::from_translation(level_to_world_pos(pos, 0.0))
                        .with_scale(Vec3::splat(4.0)),
                    Sprite::from_image(a.load(format!("textures/blocks/{}.png", block.texture_id))) 
                )).id();
            } else {
                block_entity = c.spawn((
                    Transform::from_translation(level_to_world_pos(pos, 0.0))
                        .with_scale(Vec3::splat(4.0)),
                    Sprite::from_image(a.load(format!("textures/blocks/{}.png", block.texture_id))),
                    Collider::cuboid(block.colider_size.0, block.colider_size.1)
                )).id();
            }

            match block.layer {
                block::BlockLayer::Floor => {
                    self.remove_block(c, pos, block::BlockLayer::Floor);
                    self.floor_layer.insert(pos, (id, block_entity));
                },
                block::BlockLayer::Wall => {
                    self.remove_block(c, pos, block::BlockLayer::Wall);
                    self.wall_layer.insert(pos, (id, block_entity));
                }
            }
        }
    }

    pub fn remove_block(
        &mut self, c: &mut Commands,
        pos: IVec2, layer: block::BlockLayer,
    ) -> bool {
        match layer {
            block::BlockLayer::Floor => {
                if self.floor_layer.contains_key(&pos) {
                    let block = self.floor_layer.get(&pos).unwrap().to_owned();
                    c.entity(block.1).despawn();
                    self.floor_layer.remove(&pos);

                    return true;
                }
            },
            block::BlockLayer::Wall => {
                if self.wall_layer.contains_key(&pos) {
                    let block = self.wall_layer.get(&pos).unwrap().to_owned();
                    c.entity(block.1).despawn();
                    self.wall_layer.remove(&pos);

                    return true;
                }
            }
        }

        false
    }

    pub fn get_block(
        &self, pos: IVec2, layer: block::BlockLayer
    ) -> Option<usize> {
        match layer {
            block::BlockLayer::Floor => {
                if self.floor_layer.contains_key(&pos) {
                    return Some(self.floor_layer.get(&pos).unwrap().0);
                }
            },
            block::BlockLayer::Wall => {
                if self.wall_layer.contains_key(&pos) {
                    return Some(self.wall_layer.get(&pos).unwrap().0);
                }
            }
        }

        None
    }

    pub fn print_blocks(&self) {
        println!("Floor");

        for floor in &self.floor_layer {
            println!("pos: {}, id: {}", floor.0, floor.1.0);
        }

        println!("Wall");

        for wall in &self.wall_layer {
            println!("pos: {}, id: {}", wall.0, wall.1.0);
        }

        println!("===");
    }

    pub fn set_color_block(
        &mut self, c: &mut Commands,
        pos: IVec2, color: Color
    ) {
        let noise_block = c.spawn((
            Transform::from_translation(level_to_world_pos(pos, 0.0))
                .with_scale(Vec3::splat(4.0)),
            Sprite::from_color(color, vec2(16.0, 16.0)) 
        )).id();

        self.floor_layer.insert(pos, (1, noise_block));
    }
}
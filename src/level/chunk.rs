use bevy::prelude::*;
use super::*;

pub const CHUNK_SIZE: i32 = 16;

pub fn level_pos_to_chunk_id(lpos: IVec2) -> IVec2 {
    ivec2(
        lpos.x / CHUNK_SIZE,
        lpos.y / CHUNK_SIZE
    )
}

pub fn chunk_id_to_level_pos(cid: IVec2) -> IVec2 {
    ivec2(
        cid.x * CHUNK_SIZE,
        cid.y * CHUNK_SIZE
    )
}

pub struct Chunk {
    pub floor_layer: HashMap<IVec2, (usize, Entity)>,
    pub wall_layer: HashMap<IVec2, (usize, Entity)>,
}

#[allow(dead_code)]
impl Chunk {
    fn new() -> Self {
        Self {
            floor_layer: HashMap::new(),
            wall_layer: HashMap::new(),
        }
    }

    pub fn build(&mut self, level: &Level, c: &mut Commands, a: &AssetServer) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                self.set_block(level, c, a, ivec2(x, y), 1);
            }
        }
    }

    pub fn set_block(
        &mut self, level: &Level, c: &mut Commands, a: &AssetServer,
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
use bevy::prelude::*;
use rand::prelude::*;
use crate::{config, level::biome::{moss_gardens::MossGardens, Biome}};

use super::*;

pub fn build_level(
    mut c: Commands, 
    mut level: ResMut<Level>,
    a: Res<AssetServer>,
    config: Res<config::GameConfig>,
) {
    let mut rng = rand::rng();
    let seed = rng.next_u32();

    info!("Level seed: [{}]", seed);
    info!("Level size: [{}x{}]", LEVEL_SIZE, LEVEL_SIZE);

    for x in 0..LEVEL_SIZE {
        for y in 0..LEVEL_SIZE {
            let pos = IVec2::new(x, y);
            level.set_block(&mut c, &a, pos, MossGardens::get_floor(pos, seed));

            if let Some(wall) = MossGardens::get_wall(pos, seed) {
                level.set_block(&mut c, &a, pos, wall);
            }
        }
    }

    if config.experiments_enabled && config.experiments.structure_gen {
        let structure = structure::fetch_structure_data("basalt_temple");
        let pos_x = rng.random_range(0..LEVEL_SIZE - structure.width);
        let pos_y = rng.random_range(0..LEVEL_SIZE - structure.height);

        let offset = IVec2::new(pos_x, pos_y);

        info!("Structure offset: {}", offset);
        
        for floor in structure.floors {
            level.set_block(&mut c, &a, IVec2::new(floor.0.0, floor.0.1) + offset, floor.1);
        }

        for wall in structure.walls {
            level.set_block(&mut c, &a, IVec2::new(wall.0.0, wall.0.1) + offset, wall.1);
        }
    }
}
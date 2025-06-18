use bevy::prelude::*;
use rand::prelude::*;
use crate::{config, level::structure::{basalt_temple::BasaltTemple, Structure}};

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
    info!("Biome noise freq: [{}]", biome::get_freq());

    for x in 0..LEVEL_SIZE {
        for y in 0..LEVEL_SIZE {
            let pos = ivec2(x, y);
            let biome: Box<dyn biome::Biome + 'static>;

            if config.experiments_enabled && config.experiments.flat_world {
                biome = biome::get_biomes().get(2).unwrap().clone_box();
            } else {
                biome = biome::get_biome_at_pos(pos, seed);
            }

            level.set_block(&mut c, &a, pos, biome.get_floor(pos, seed));

            if let Some(wall) = biome.get_wall(pos, seed) {
                level.set_block(&mut c, &a, pos, wall);
            }
        }
    }

    if config.experiments_enabled && config.experiments.structure_gen {
        let pos_x = rng.random_range(0..LEVEL_SIZE - BasaltTemple::get_size().x);
        let pos_y = rng.random_range(0..LEVEL_SIZE - BasaltTemple::get_size().y);
        let offset = ivec2(pos_x, pos_y);

        info!("Structure offset: {}", offset);

        BasaltTemple::build(&mut c, &a, offset, &mut level);
    }
}
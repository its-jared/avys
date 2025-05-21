use bevy::prelude::*;
use rand::prelude::*;
use crate::level::biome::{moss_gardens::MossGardens, Biome};

use super::*;

pub fn build_level(
    mut c: Commands, 
    mut level: ResMut<Level>,
    a: Res<AssetServer>,
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
}
use bevy::prelude::*;
use rand::prelude::*;

use super::*;

pub fn build_level(
    mut c: Commands, 
    mut level: ResMut<Level>,
    a: Res<AssetServer>,
) {
    let mut rng = rand::rng();
    let seed = rng.next_u32();

    info!("Level seed: [{}]", seed);
    info!("Biome noise freq: [{}]", biome::get_freq());

    level.build_chunk_at(&mut c, &a, ivec2(0, 0));
    level.build_chunk_at(&mut c, &a, ivec2(16, 0));
}
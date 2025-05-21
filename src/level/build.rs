use bevy::prelude::*;
use rand::prelude::*;
use noise::{NoiseFn, Perlin};
use super::*;

pub fn build_level(
    mut c: Commands, 
    mut level: ResMut<Level>,
    a: Res<AssetServer>,
) {
    let mut rng = rand::rng();
    let seed = rng.next_u32();
    let perlin = Perlin::new(seed);

    info!("Level seed: [{}]", seed);
    info!("Level size: [{}x{}]", LEVEL_SIZE, LEVEL_SIZE);

    for x in 0..LEVEL_SIZE {
        for y in 0..LEVEL_SIZE {
            let v = perlin.get([
                x as f64 / 1000.0,
                y as f64 / 1000.0,
                0.5
            ]);
            let mut block = 1;

            if v < 0.0 {
                block = 2;
            }

            level.set_block(&mut c, &a, IVec2::new(x, y), block);
        }
    }
}
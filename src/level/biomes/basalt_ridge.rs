use super::biome::*;
use bevy::prelude::*;
use bracket_noise::prelude::*;
use rand::prelude::*;

pub struct BasaltRidge;

impl Biome for BasaltRidge {
    fn get_block(&self, pos: IVec2) -> usize {
        let mut rnd = rand::rng();
        let mut noise = FastNoise::seeded(10);
        noise.set_noise_type(NoiseType::Simplex);
        noise.set_frequency(0.1);

        let nv = noise.get_noise(pos.x as f32, pos.y as f32);
        let rv = rnd.random_range(0..100);

        if nv <= 0.0 && rv <= 25 {
            return 3;
        } else if nv >= 1.0 {
            return 4;
        }

        2
    }
}
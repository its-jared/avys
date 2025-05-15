use super::biome::*;
use bevy::prelude::*;
use bracket_noise::prelude::*;
use rand::prelude::*;

pub struct BasaltRidge;

impl Biome for BasaltRidge {
    fn get_ground(&self, _pos: IVec2) -> String {
        "basalt".to_string()
    }

    fn get_decor(&self, pos: IVec2) -> String {
        let mut rnd = rand::rng();
        let mut noise = FastNoise::seeded(10);
        noise.set_noise_type(NoiseType::Simplex);
        noise.set_frequency(0.01);

        let nv = noise.get_noise(pos.x as f32, pos.y as f32);
        let rv = rnd.random_range(0..100);

        if nv >= 1.0 {
            return "basalt_wall".to_string();
        }
        else if nv <= 0.0 && rv <= 5 {
            return "ember_ore".to_string();
        }       

        "air".to_string()
    }

    fn get_name(&self) -> String {
        "Basalt Ridge".to_string()
    }
}
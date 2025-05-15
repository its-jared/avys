use super::{basalt_ridge::BasaltRidge, moss_garden::MossGarden};
use bevy::prelude::*;
use bracket_noise::prelude::*;

pub trait Biome {
    fn get_ground(&self, pos: IVec2) -> String;
    fn get_decor(&self, pos: IVec2) -> String;
    fn get_name(&self) -> String;
}

pub struct BiomeRegistery {
    pub biomes: Vec<Box<dyn Biome>>,
}

pub fn get_registery() -> BiomeRegistery {
    BiomeRegistery { 
        biomes: vec![
            Box::new(BasaltRidge),
            Box::new(MossGarden),
        ]
    }
}

pub fn get_biome(pos: IVec2, registery: &BiomeRegistery) -> &Box<dyn Biome + 'static> {
    let mut noise = FastNoise::seeded(10);
    noise.set_noise_type(NoiseType::Simplex);
    noise.set_frequency(0.001);

    let v = noise.get_noise(pos.x as f32, pos.y as f32);
    let mut biome_index = 0;

    if v <= 0.0 {
        biome_index = 1;
    }

    registery.biomes.get(biome_index).unwrap()
}
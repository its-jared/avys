use super::{basalt_ridge::BasaltRidge, moss_garden::MossGarden};
use bevy::prelude::*;

pub trait Biome {
    fn get_block(&self, pos: IVec2) -> usize;
}

pub struct BiomeRegistery {
    pub biomes: Vec<Box<dyn Biome>>,
}

pub fn set_registery() -> BiomeRegistery {
    BiomeRegistery { 
        biomes: vec![
            Box::new(BasaltRidge),
            Box::new(MossGarden),
        ]
    }
}
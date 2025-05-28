use bevy::prelude::*;
use noise::{Worley, NoiseFn};
use super::*;

pub mod moss_gardens;
pub mod basalt_ridge;
pub mod flat_lands;
pub mod desert;

pub fn get_biomes() -> Vec<Box<dyn Biome>> {
    vec![
        Box::new(moss_gardens::MossGardens),
        Box::new(basalt_ridge::BasaltRidge),
        Box::new(flat_lands::FlatLands),
        Box::new(desert::Desert),
    ]
}

pub fn get_freq() -> f64 {
    (LEVEL_SIZE / get_biomes().len() as i32) as f64
}

pub fn get_biome_at_pos(pos: IVec2, seed: u32) -> Box<dyn Biome + 'static> {
    let biomes = get_biomes();
    let worley = Worley::new(seed);
    let v = worley.get([
        pos.x as f64 / get_freq(),
        pos.y as f64 / get_freq(),
        0.53,
    ]);

    if v > 0.0 { return biomes.get(0).unwrap().clone_box(); }
    biomes.get(3).unwrap().clone_box()
}

pub trait Biome: 'static + BiomeClone + Sync + Send {
    fn get_floor(&self, pos: IVec2, seed: u32) -> usize;
    fn get_wall(&self, pos: IVec2, seed: u32) -> Option<usize>;
}

pub trait BiomeClone {
    fn clone_box(&self) -> Box<dyn Biome>;
}

impl<T> BiomeClone for T
where
    T: 'static + Biome + Clone,
{
    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Biome> {
    fn clone(&self) -> Box<dyn Biome> {
        self.clone_box()
    }
}
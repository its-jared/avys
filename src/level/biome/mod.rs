use bevy::prelude::*;

pub mod moss_gardens;
pub mod basalt_ridge;

pub trait Biome {
    fn get_floor(pos: IVec2, seed: u32) -> usize;
    fn get_wall(pos: IVec2, seed: u32) -> Option<usize>;
}
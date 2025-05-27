use bevy::prelude::*;
use super::*;

#[derive(Clone)]
pub struct FlatLands; 

impl Biome for FlatLands {
    fn get_floor(&self, _pos: IVec2, _seed: u32) -> usize {
        1
    }

    fn get_wall(&self, _pos: IVec2, _seed: u32) -> Option<usize> {
        None
    }
}
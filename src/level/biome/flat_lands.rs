use bevy::prelude::*;
use super::*;

#[derive(Clone)]
pub struct FlatLands; 

impl Biome for FlatLands {
    fn get_floor(&self, _pos: IVec2, _seed: u32) -> String {
        "avys:moss".to_string()
    }

    fn get_wall(&self, _pos: IVec2, _seed: u32) -> Option<String> {
        None
    }
}
use bevy::prelude::*;
use rand::Rng;
use super::*;

#[derive(Clone)]
pub struct Desert; 

impl Biome for Desert {
    fn get_floor(&self, _pos: IVec2, _seed: u32) -> usize {
        10
    }

    fn get_wall(&self, _pos: IVec2, _seed: u32) -> Option<usize> {
        let mut rng = rand::rng();
        let r = rng.random_range(0..100);

        if r < 5 {
            return Some(11);
        }
        
        None
    }
}
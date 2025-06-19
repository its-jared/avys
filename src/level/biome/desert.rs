use bevy::prelude::*;
use rand::Rng;
use super::*;

#[derive(Clone)]
pub struct Desert; 

impl Biome for Desert {
    fn get_floor(&self, _pos: IVec2, _seed: u32) -> String {
        "avys:sand".to_string()
    }

    fn get_wall(&self, _pos: IVec2, _seed: u32) -> Option<String> {
        let mut rng = rand::rng();
        let r = rng.random_range(0..100);

        if r < 5 {
            return Some("avys:cactus".to_string());
        }
        
        None
    }
}
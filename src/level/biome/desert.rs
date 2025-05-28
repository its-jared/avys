use bevy::prelude::*;
use noise::Perlin;
use rand::Rng;
use super::*;

#[derive(Clone)]
pub struct Desert; 

impl Biome for Desert {
    fn get_floor(&self, pos: IVec2, seed: u32) -> usize {
        let perlin = Perlin::new(seed);

        let v = perlin.get([
            pos.x as f64 / 50.0,
            pos.y as f64 / 50.0,
            0.5
        ]);

        if v < -0.2 {
            return 13;
        }

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
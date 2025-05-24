use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use super::*;

#[derive(Clone)]
pub struct MossGardens; 

impl Biome for MossGardens {
    fn get_floor(&self, pos: IVec2, seed: u32) -> usize {
        let perlin = Perlin::new(seed);
        let v = perlin.get([
            pos.x as f64 / 5.0,
            pos.y as f64 / 5.0,
            0.2
        ]);

        if v < -0.3 { return 7; }
        
        1
    }

    fn get_wall(&self, pos: IVec2, seed: u32) -> Option<usize> {
        let perlin = Perlin::new(seed);
        let mut rng = rand::rng();

        let r = rng.random_range(0..100);
        let v = perlin.get([
            pos.x as f64 / 50.0,
            pos.y as f64 / 50.0,
            0.5
        ]);

        if v >= 0.0 {
            if r < 5 {
                return Some(5);
            } else if r < 10 {
                return Some(4);
            }
        }

        None
    }
}
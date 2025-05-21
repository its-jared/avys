use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use super::*;

pub struct MossGardens; 

impl Biome for MossGardens {
    fn get_floor(_pos: IVec2, _seed: u32) -> usize {
        1
    }

    fn get_wall(pos: IVec2, seed: u32) -> Option<usize> {
        let perlin = Perlin::new(seed);
        let mut rng = rand::rng();

        let r = rng.random_range(0..100);
        let v = perlin.get([
            pos.x as f64 / 50.0,
            pos.y as f64 / 50.0,
            0.5
        ]);

        if v >= 0.0 {
            if r < 10 {
                return Some(4);
            } else if r < 25 {
                return Some(5);
            }
        }

        None
    }
}
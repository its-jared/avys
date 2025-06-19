use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use super::*;

#[derive(Clone)]
pub struct BasaltRidge; 

impl Biome for BasaltRidge {
    fn get_floor(&self, _pos: IVec2, _seed: u32) -> String {
        "avys:basalt".to_string()
    }

    fn get_wall(&self, pos: IVec2, seed: u32) -> Option<String> {
        let perlin = Perlin::new(seed);
        let mut rng = rand::rng();

        let r = rng.random_range(0..100);
        let v1 = perlin.get([
            pos.x as f64 / 50.0,
            pos.y as f64 / 50.0,
            0.5
        ]);

        let v2 = perlin.get([
            pos.x as f64 / 5.0,
            pos.y as f64 / 5.0,
            0.2
        ]);

        if v1 < 0.0 {
            if r < 5 {
                return Some("avys:rock".to_string());
            }
        }

        if v2 < -0.2 {
            return Some("avys:basalt_wall".to_string())
        }

        None
    }
}
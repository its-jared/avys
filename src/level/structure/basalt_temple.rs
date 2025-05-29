/*
    Basalt Wall: 3,
    Basalt Tile: 8,
    Polished Basalt: 9,
*/

use bevy::prelude::*;
use rand::Rng;
use super::*;

pub struct BasaltTemple;

const SIZE: i32 = 7;

impl Structure for BasaltTemple {
    fn build(c: &mut Commands, a: &AssetServer, 
        offset: IVec2, level: &mut Level) {
        let design: Vec<Vec<usize>>;
        let mut rng = rand::rng();
        let face = rng.random_range(0..3); 

        match face {
            0 => {
                design = vec![
                    vec![3, 3, 3, 9, 3, 3, 3],
                    vec![3, 9, 9, 9, 9, 9, 3],
                    vec![3, 9, 8, 8, 8, 9, 3],
                    vec![3, 9, 9, 9, 9, 9, 3],
                    vec![3, 3, 3, 3, 3, 3, 3],
                ];
            },
            1 => {
                design = vec![
                    vec![3, 3, 3, 3, 3, 3, 3],
                    vec![3, 9, 9, 9, 9, 9, 3],
                    vec![3, 9, 8, 8, 8, 9, 3],
                    vec![3, 9, 9, 9, 9, 9, 3],
                    vec![3, 3, 3, 9, 3, 3, 3],
                ];
            },
            2 => {
                design = vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 9, 9, 9, 3],
                    vec![3, 9, 8, 9, 3],
                    vec![9, 9, 8, 9, 3],
                    vec![3, 9, 8, 9, 3],
                    vec![3, 9, 9, 9, 3],
                    vec![3, 3, 3, 3, 3],
                ];
            },
            3 | _ => {
                design = vec![
                    vec![3, 3, 3, 3, 3],
                    vec![3, 9, 9, 9, 3],
                    vec![3, 9, 8, 9, 3],
                    vec![3, 9, 8, 9, 9],
                    vec![3, 9, 8, 9, 3],
                    vec![3, 9, 9, 9, 3],
                    vec![3, 3, 3, 3, 3],
                ];
            }
        }

        for x in 0..design.len() {
            for y in 0..design.get(x).unwrap().len() {
                //level.set_block(c, a, ivec2(x as i32, y as i32) + offset, design[x][y]);
            }
        }
    }

    fn get_size() -> IVec2 {
        ivec2(SIZE, SIZE)
    }
}
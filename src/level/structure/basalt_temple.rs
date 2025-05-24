use bevy::prelude::*;
use super::*;

pub struct BasaltTemple;

const SIZE: i32 = 25;

impl Structure for BasaltTemple {
    fn build(c: &mut Commands, a: &AssetServer, 
        offset: IVec2, level: &mut Level) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                let pos = offset + ivec2(x, y);
                level.set_block(c, a, pos, 8);
            }
        }
    }

    fn get_size() -> IVec2 {
        ivec2(SIZE, SIZE)
    }
}
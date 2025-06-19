use bevy::prelude::*;
use super::*;

pub trait Structure {
    fn build(c: &mut Commands, a: &AssetServer, offset: IVec2, level: &mut Level);
    fn get_size() -> IVec2;
}
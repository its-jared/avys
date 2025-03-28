use std::fs;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const BLOCK_DATA_PATH: &str = "assets/data/blocks.ron";

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub desc: String,
    pub img_path: String,
    pub alpha: f32,
    pub solid: bool,
}

#[derive(Resource, Debug, Deserialize, Serialize)]
pub struct BlockResource(pub Vec<Block>);

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_block_resource);
    }
}

fn init_block_resource(mut commands: Commands) {
    info!("Loading blocks from {}", BLOCK_DATA_PATH);   

    let cont = fs::read_to_string(BLOCK_DATA_PATH)
        .expect(format!("Error when attempting to read from {}", BLOCK_DATA_PATH).as_str());
    let val: BlockResource = ron::from_str(cont.as_str()).unwrap();

    commands.insert_resource(val);
}
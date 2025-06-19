use std::{collections::HashMap, fs};
use serde::Deserialize;

#[derive(Deserialize)]
pub enum BlockLayer {
    Floor,
    Wall
}

#[derive(Deserialize, PartialEq, Eq)]
pub enum BlockType {
    Block,
    Solid,
    Liquid,
}

#[derive(Deserialize)]
pub struct Block {
    pub name: String,
    pub id: i32, 
    pub random_texture: bool,
    pub textures: Vec<String>,
    pub block_type: BlockType,
    pub colider_size: (f32, f32),
    pub layer: BlockLayer,
}

pub fn get_blocks() -> HashMap<String, Block> {
    let data_path: &str = "assets/data/blocks.ron";
    
    println!("Loading Blocks from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from blocks.ron file!");
    let val: HashMap<String, Block> = ron::from_str(raw.as_str()).unwrap();

    val
}
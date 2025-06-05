use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum BlockLayer {
    Floor,
    Wall
}

#[derive(Deserialize)]
pub struct Block {
    pub name: String,
    pub id: i32, 
    pub texture_id: String, 
    pub solid: bool,
    pub colider_size: f32,
    pub layer: BlockLayer,
}

pub fn get_blocks() -> Vec<Block> {
    let data_path: &str = "assets/data/blocks.ron";
    
    println!("Loading Blocks from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from blocks.ron file!");
    let val: Vec<Block> = ron::from_str(raw.as_str()).unwrap();

    val
}
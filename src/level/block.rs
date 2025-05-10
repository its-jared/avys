use std::fs;
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Block {
    pub name: String, 
    pub id: i32, 
    pub texture: String, 
}

impl Default for Block {
    fn default() -> Self {
        Self {
            name: "Default Block".to_string(),
            id: -1,
            texture: "basalt".to_string(),
        }
    }
}

#[derive(Resource, Clone, Deserialize)]
pub struct BlockRegistery {
    pub blocks: Vec<Block>,
}

pub fn get_registery() -> BlockRegistery {
    let data_path: &str = "assets/data/blocks.ron";
    
    info!("Loading Block Registery from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from world_gen file!");
    let val: BlockRegistery = ron::from_str(raw.as_str()).unwrap();

    val
}
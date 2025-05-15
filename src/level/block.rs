use std::{collections::HashMap, fs};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum BlockLayer {
    Ground,
    Decor, 
}

#[derive(Clone, Deserialize)]
pub struct Block {
    pub name: String, 
    pub texture: String, 
    pub random_rotation: bool,
    pub multiple_textures: bool,
    pub textures: Vec<String>,
    pub layer: BlockLayer
}

impl Default for Block {
    fn default() -> Self {
        Self {
            name: "Default Block".to_string(),
            texture: "basalt".to_string(),
            random_rotation: false,
            multiple_textures: false,
            textures: Vec::new(),
            layer: BlockLayer::Decor,
        }
    }
}

#[derive(Resource, Clone, Deserialize)]
pub struct BlockRegistery {
    pub blocks: HashMap<String, Block>,
}

pub fn get_registery() -> BlockRegistery {
    let data_path: &str = "assets/data/blocks.ron";
    
    info!("Loading Block Registery from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from blocks.ron file!");
    let val: BlockRegistery = ron::from_str(raw.as_str()).unwrap();

    val
}
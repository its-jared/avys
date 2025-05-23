use std::{collections::HashMap, fs};
use serde::Deserialize;
use bevy::prelude::*;

#[derive(Deserialize)]
pub struct StructureData {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub floors: HashMap<(i32, i32), usize>,
    pub walls: HashMap<(i32, i32), usize>,
}

pub fn fetch_structure_data(name: &str) -> StructureData {
    let data_path = format!("assets/data/structures/{}.ron", name);
    
    info!("Loading structure from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from structure's ron file!");
    let val: StructureData = ron::from_str(raw.as_str()).unwrap();

    val
}
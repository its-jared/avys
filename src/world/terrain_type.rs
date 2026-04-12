use std::{collections::HashMap, fs};
use bevy::prelude::*;
use serde::Deserialize;
use walkdir::WalkDir;

#[derive(Resource)]
pub struct CurrentTerrainType(pub String);
#[derive(Resource)]
pub struct TerrainTypes(pub HashMap<String, TerrainType>);

#[derive(Deserialize)]
pub struct TerrainType {
    pub name: String,
    pub description: String,

    pub size: (u32, u32),
    pub noise_scale: f64,
    pub height_scale: f32,

    pub has_water: bool,
    pub water_color: Option<(f32, f32, f32, f32)>,
    pub water_height: Option<f32>,
}

pub fn collect_terrain_types() -> HashMap<String, TerrainType> {
    let mut types: HashMap<String, TerrainType> = HashMap::new();

    for entry in WalkDir::new("assets/data/terrain_types/") {
        let entry = entry.expect("Error when loading terrain types path!");

        if entry.path().extension().and_then(|e| e.to_str()) == Some("ron") {
            let name = entry.file_name().to_str().unwrap();
            let id: Vec<&str> = name.split('.').collect();
            
            let contents = fs::read_to_string(entry.path())
                .expect("Error when reading terrain type content!");
            let terrain_type: TerrainType = ron::from_str(contents.as_str())
                .expect("Error when parsing terrain type content into TerrainType struct!");
            
            types.insert(id[0].into(), terrain_type);
        }
    }

    types
}
use std::fs;
use serde::Deserialize;
use bevy::prelude::*;

#[derive(Deserialize, Debug)]
pub struct GameExperiments {
    pub structure_gen: bool,
}

#[derive(Resource, Deserialize)]
pub struct GameConfig {
    pub version: (i32, i32, i32),
    pub experiments_enabled: bool,
    pub experiments: GameExperiments,
}

pub fn fetch_game_config() -> GameConfig {
    let data_path: &str = "assets/data/config.ron";
    
    println!("Loading GameConfig from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from config.ron file!");
    let val: GameConfig = ron::from_str(raw.as_str()).unwrap();

    if val.experiments_enabled {
        println!("Experiments are enabled: {:?}", val.experiments);
    }

    val
}
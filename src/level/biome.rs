use std::fs;
use bracket_noise::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

pub struct Biome {
    pub name: String, 
    pub id: u32, 
    
    pub moisture_range: (f32, f32),
    pub temperature_range: (f32, f32),
    pub height_range: (f32, f32),

    pub ground_color: (f32, f32, f32),
}

#[derive(Resource, Deserialize)]
pub struct BiomeRegistery(pub Vec<Biome>);

pub fn get_biome_noises(pos: IVec2) -> (f32, f32, f32) {
    let mut noise_moisture = FastNoise::seeded(10);
    noise_moisture.set_noise_type(NoiseType::Cubic);
    noise_moisture.set_frequency(0.01);

    let mut noise_temperature = FastNoise::seeded(10);
    noise_temperature.set_noise_type(NoiseType::Cubic);
    noise_temperature.set_frequency(0.01);

    let mut noise_height = FastNoise::seeded(10);
    noise_height.set_noise_type(NoiseType::Cubic);
    noise_height.set_frequency(0.01);

    (
        noise_moisture.get_noise(pos.x as f32, pos.y as f32),
        noise_temperature.get_noise(pos.x as f32, pos.y as f32),
        noise_height.get_noise(pos.x as f32, pos.y as f32)
    )
}

pub fn get_biome(pos: IVec2, biomes: Vec<Biome>) -> Biome {
    let n = get_biome_noises(pos);
    
    for biome in biomes {
        if n.0 < biome.moisture_range.0 && n.0 > biome.moisture_range.1 {
            continue;
        }

        if n.1 < biome.temperature_range.0 && n.1 > biome.temperature_range.1 {
            continue;
        }

        if n.2 < biome.height_range.0 && n.2 > biome.height_range.1 {
            continue;
        }

        return biome;
    }

    Biome {
        name: "Empty Biome - No Canidates Found!".to_string(),
        id: 0, 

        moisture_range: (0.0, 0.0),
        temperature_range: (0.0, 0.0),
        height_range: (0.0, 0.0),

        ground_color: (1.0, 0.0, 1.0)
    }
}

fn setup_registery(mut cmds: Commands) {
    let data_path: &str = "assets/data/biomes.ron";
    
    println!("Loading Biome Registery from: {}", data_path);

    let raw = fs::read_to_string(data_path)
        .expect("Error when reading raw string from world_gen file!");
    let val: BiomeRegistery = ron::from_str(raw.as_str()).unwrap();

    cmds.insert_resource(val);
}

pub struct BiomePlugin; 

impl Plugin for BiomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_registery);
    }
}
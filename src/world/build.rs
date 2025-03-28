use bevy::prelude::*;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use super::{world_commands::WorldCommands, *};

pub fn build_world(mut commands: Commands) {
    info!("Building world...");

    let mut noise = FastNoise::new(); 
    noise.set_noise_type(NoiseType::CubicFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.3);
    noise.set_fractal_lacunarity(1.0);
    noise.set_frequency(2.0);

    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            let mod_val = noise.get_noise(x as f32 / 50., y as f32 / 500.) * 5.;
            let val = noise.get_noise(x as f32 / 100. * mod_val, y as f32 / 100. * mod_val) * 2.;

            if val >= 0.0 {
                commands.set_block(1, Vec3::new(x as f32, y as f32, 0.0));
            }
        }
    }
}
use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers::square_grid::neighbors::Neighbors, map::*, prelude::get_tilemap_center_transform, tiles::*, TilemapBundle};
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};

// Reference: https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/basic.rs
pub fn build_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Building world...");

    let mut noise = FastNoise::new(); 
    noise.set_noise_type(NoiseType::CubicFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.3);
    noise.set_fractal_lacunarity(1.0);
    noise.set_frequency(2.0);
    
    let texture_handle: Handle<Image> = asset_server.load("textures/tiles.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let mod_val = noise.get_noise(x as f32 / 50., y as f32 / 500.) * 5.;
            let val = noise.get_noise(x as f32 / 100. * mod_val, y as f32 / 100. * mod_val) * 2.;

            if val >= 0.0 {
                
            }

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

/*pub fn build_world(mut commands: Commands) {
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
}*/
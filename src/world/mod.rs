use bevy::prelude::*;

use crate::world::{generation::generate_terrain_mesh, terrain_type::{CurrentTerrainType, TerrainTypes}};

pub mod generation;
pub mod terrain_type;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, generate);
    }
}

fn generate(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_types: Res<TerrainTypes>,
    current_terrain_type: Res<CurrentTerrainType>,
) {
    let Some(terrain_data) = terrain_types.0.get(&current_terrain_type.0) else { 
        error!("Unable to find terrain type by the id of {}!", current_terrain_type.0);
        return;
    };

    let mesh = generate_terrain_mesh(
        terrain_data.size.0,
        terrain_data.size.1,
        terrain_data.noise_scale,
        terrain_data.height_scale,
        rand::random_range(0..999999999),
    );
    let terrain_material = StandardMaterial {
        base_color: Color::oklch(0.35, 0.06, 59.0),
        metallic: 0.0,
        perceptual_roughness: 1.0,
        ..default()
    };

    c.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(terrain_material)),
        Transform::from_xyz(-(terrain_data.size.0 as f32 / 2.0) * 4.0, 0.0, -(terrain_data.size.1 as f32 / 2.0) * 4.0)
            .with_scale(Vec3::splat(4.0))
    ));

    if terrain_data.has_water {
        let water_color = terrain_data.water_color.unwrap();

        let water_material = StandardMaterial {
            base_color: Color::srgba(water_color.0, water_color.1, water_color.2, water_color.3),
            alpha_mode: AlphaMode::AlphaToCoverage,
            ..default()
        };

        c.spawn((
            Mesh3d(meshes.add(Cuboid::new(terrain_data.size.0 as f32 - 0.001, 60.0, terrain_data.size.1 as f32 - 0.001))),
            MeshMaterial3d(materials.add(water_material)),
            Transform::from_xyz(-0.0 * 4.0, (-60.0 + terrain_data.water_height.unwrap()) * 4.0, -0.0 * 4.0)
                .with_scale(Vec3::splat(4.0))
        ));
    }

    c.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: false,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            ..default()
        },
    ));

}
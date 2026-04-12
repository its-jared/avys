use bevy::{light::PointLightShadowMap, prelude::*};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use crate::{player::PlayerPlugin, world::WorldPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PanOrbitCameraPlugin,

                WorldPlugin,
                PlayerPlugin,
            ))
            .insert_resource(GlobalAmbientLight {
                color: Color::WHITE,
                ..default()
            })
            .insert_resource(PointLightShadowMap {
                size: 64,
            });
    }
}
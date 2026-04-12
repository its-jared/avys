use bevy::{camera::ScalingMode, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, TrackpadBehavior};

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct PlayerStats {
    pub num_of_wins: i32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut c: Commands) {
    c.spawn((
        Player,
        PlayerStats {
            num_of_wins: 0
        },
        
        Transform::from_translation(Vec3::new(50.0, 35.0, 50.0)).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera {
            pan_sensitivity: 0.0,

            pitch_lower_limit: Some(0.0),

            zoom_lower_limit: 25.0,
            zoom_upper_limit: Some(150.0),
            
            trackpad_pinch_to_zoom_enabled: true,

            trackpad_behavior: TrackpadBehavior::blender_default(),
            ..default()
        },

        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: false,
            ..default()
        },
    ));
}
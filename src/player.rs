use bevy::prelude::*;
use bevy_flycam::FlyCam;

#[derive(Component)]
pub struct PlayerCharacter;

fn build_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::new(1., 2., 1.)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            ..default()
        })),
        PlayerCharacter,
        Transform::from_xyz(0., 10., 0.),
    ));
}

fn update_player(
    q_cam: Query<&Transform, With<FlyCam>>,
    mut q_player: Query<(&mut Transform, &PlayerCharacter), Without<FlyCam>>,
) {
    let cam_trans = q_cam.single();
    let mut player_trans = q_player.single_mut();
    
    player_trans.0.translation = 
        Vec3::new(
            cam_trans.translation.x,
            10.,
            cam_trans.translation.z
        );
}

pub struct PlayerCharacterPlugin;
impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_player);
        app.add_systems(Update, update_player);
    }
}
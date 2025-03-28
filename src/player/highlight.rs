use bevy::prelude::*;
use crate::world::{world_to_global_pos, TILE_SIZE};
use super::{Face, Player};

#[derive(Component)]
pub struct Highlight;

#[derive(Resource)]
pub struct HighlightPos(pub Vec3); // this is in world not global space. 

pub fn spawn_highlight(
    mut commands: Commands, 
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.5), 
            custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
            ..Default::default()
        },
        Highlight,
    ));

    commands.insert_resource(HighlightPos(Vec3::ZERO));
}

pub fn update_highlight(
    mut highlight_q: Query<&mut Transform, With<Highlight>>,
    mut highlight_pos: ResMut<HighlightPos>,
    player_q: Query<(&Player, &Transform), Without<Highlight>>,
) {
    let mut highlight = highlight_q.single_mut();
    let player = player_q.single();
    
    let pos: Vec3;

    match player.0.facing {
        Face::North => { pos = Vec3::new(0.0, 1.0, 0.0); }
        Face::South => { pos = Vec3::new(0.0, -1.0, 0.0); }
        Face::East => { pos = Vec3::new(1.0, 0.0, 0.0); }
        Face::West => { pos = Vec3::new(-1.0, 0.0, 0.0); }
    }

    let translation = pos + player.0.pos;
    highlight_pos.0 = translation;
    highlight.translation = world_to_global_pos(translation);
}
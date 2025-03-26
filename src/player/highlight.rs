use bevy::prelude::*;

use crate::world::{world_to_global_pos, TILE_SIZE};

use super::{Face, Player};

#[derive(Component)]
pub struct Highlight;

pub fn spawn_highlight(
    mut commands: Commands, 
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            color: Color::WHITE, 
            custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
            ..Default::default()
        },
        Highlight,
    ));
}

pub fn update_highlight(
    mut highlight_q: Query<&mut Transform, With<Highlight>>,
    player_q: Query<(&Player, &Transform), Without<Highlight>>,
) {
    let mut highlight = highlight_q.single_mut();
    let player = player_q.single();
    
    let pos: Vec3;

    match player.0.facing {
        Face::North => { pos = Vec3::new(0.0, 1.0, 0.0); }
        Face::South => { pos = Vec3::new(1.0, -1.0, 0.0); }
        Face::East => { pos = Vec3::new(1.0, 0.0, 0.0); }
        Face::West => { pos = Vec3::new(0.0, 0.0, 0.0); }
    }

    let translation = pos + player.0.pos;
    highlight.translation = world_to_global_pos(translation);
}
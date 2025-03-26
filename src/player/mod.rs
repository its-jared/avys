use bevy::prelude::*;
use highlight::{spawn_highlight, update_highlight};
use std::fmt;
use crate::{personality::{Health, Mood, MoodComponent}, player::controls::*};
use crate::world::{TILE_SIZE, world_to_global_pos};

pub mod controls;
pub mod player_commands;
pub mod highlight;

pub const WALKING_SPEED: f32 = 150.0;
pub const RUNNING_SPEED: f32 = 200.0;

pub enum Face {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { 
            Face::North => write!(f, "North"),
            Face::South => write!(f, "South"),
            Face::East => write!(f, "East"),
            Face::West => write!(f, "West"),
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub pos: Vec3,
    pub facing: Face,
    pub running: bool,
}

#[derive(Component)]
pub struct PlayerComponent {
    pub facing: Face,
    pub running: bool,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_player,
            spawn_highlight,
        ));
        app.add_systems(FixedUpdate, (
            keyboard_controls,
            gamepad_controls,
            update_highlight,
        ));
    }
}

fn setup_player(mut commands: Commands) {
    let center = world_to_global_pos(Vec3::new(
        (TILE_SIZE / 2) as f32, 
        (TILE_SIZE / 2) as f32, 
        0.0
    ));

    commands.spawn((
        Transform::from_xyz(center.x, center.y, 1.0),
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..Default::default()
        },
        Player {
            pos: Vec3::ZERO,
            facing: Face::North,
            running: false,
        },
        Health {
            max: 20.0,
            val: 20.0
        },
        MoodComponent(Mood::Happy),
    ));

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(center.x, center.y, 2.0),
    ));
}
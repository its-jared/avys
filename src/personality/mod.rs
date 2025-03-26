use bevy::prelude::*;
use std::fmt::Display;

#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub val: f32,
}

#[derive(Component)]
pub enum Mood {
    Happy,
    Sad,
    Neutral,
    Angry,
}

impl Display for Mood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mood::Happy => write!(f, "Happy"),
            Mood::Sad => write!(f, "Sad"),
            Mood::Neutral => write!(f, "Neutral"),
            Mood::Angry => write!(f, "Angry"),
        }
    }
}

#[derive(Component)]
pub struct Mob;

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub health: Health,
    pub mood: Mood,
    pub transform: Transform,
    pub sprite: Sprite,
}

impl Default for MobBundle {
    fn default() -> Self {
        Self {
            mob: Mob,
            health: Health {
                max: 20.0,
                val: 20.0,
            },
            mood: Mood::Neutral,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            }
        }
    }
}
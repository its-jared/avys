use bevy::ecs::{system::Commands, world::{Command, World}};
use bevy::prelude::*;
use crate::player::highlight::Highlight;

use super::TILE_SIZE;

pub struct PlaceCrop;

impl Command for PlaceCrop {
    fn apply(self, world: &mut World) {
        let mut highlight_q: QueryState<(&Transform, &Highlight)> = world.query();
        let highlight = highlight_q.single(world);

        world.spawn((
            Transform::from_translation(Vec3::new(
                highlight.0.translation.x,
                highlight.0.translation.y,
                0.0
            )),
            Sprite {
                color: Color::srgb(0.0, 1.0, 0.5),
                custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                ..Default::default()
            },
        ));
    }
}

pub trait WorldCommands {
    fn place_crop(&mut self);
}

impl<'w, 's> WorldCommands for Commands<'w, 's> { 
    fn place_crop(&mut self) {
        self.queue(PlaceCrop {});
    }
}
use std::collections::HashMap;

use bevy::prelude::*;

pub mod build;
pub mod infinite;
pub mod biomes;
pub mod block;

pub struct LevelBuilderPlugin;

impl Plugin for LevelBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(infinite::ChunkStore(HashMap::new()))
            .insert_resource(block::get_registery())

            .add_systems(
                Startup, 
                build::build_world
                    .run_if(resource_exists::<infinite::ChunkStore>)
                    .run_if(resource_exists::<block::BlockRegistery>)
            );
            /*.add_systems(
                Update,
                super::infinite::manage_chunks
                    .run_if(resource_exists::<ChunkStore>)
            );*/
    }
}
use std::collections::HashMap;
use bevy::prelude::*;
use crate::player::Player;
use super::build::{
    BuildChunk, 
    Chunk, 
    CHUNK_SIZE
};

#[derive(Resource)]
pub struct ChunkStore(pub HashMap<IVec2, Entity>);

#[allow(unused_variables)] // to be removed when this function is fully implemented.
pub fn manage_chunks( 
    mut cmds: Commands,
    mut current_chunk: Local<IVec2>,
    player: Query<&Transform, With<Player>>,
    mut chunk_store: ResMut<ChunkStore>,
    chunks: Query<
        Entity,
        With<Chunk>,
    >,
) {
    let Ok(transform) = player.single() else {
        error!("No player!");
        return;
    };

    let xz = (transform.translation.xz() / CHUNK_SIZE)
        .trunc()
        .as_ivec2();

    if *current_chunk != xz {
        *current_chunk = xz;
        let chunks_to_render = [
            *current_chunk + IVec2::new(-1, -1),
            *current_chunk + IVec2::new(-1, 0),
            *current_chunk + IVec2::new(-1, 1),
            *current_chunk + IVec2::new(0, -1),
            *current_chunk + IVec2::new(0, 0),
            *current_chunk + IVec2::new(0, 1),
            *current_chunk + IVec2::new(1, -1),
            *current_chunk + IVec2::new(1, 0),
            *current_chunk + IVec2::new(1, 1),
        ];
        
        let chunks_to_despawn: Vec<(IVec2, Entity)> =
            chunk_store
                .0
                .clone()
                .into_iter()
                .filter(|(key, _)| {
                    !chunks_to_render.contains(&key)
                })
                .collect();

        for chunk in chunks_to_despawn {
            cmds.entity(chunk.1).despawn();
            chunk_store.0.remove(&chunk.0);
        }

        for chunk in chunks_to_render {
            cmds.queue(BuildChunk(chunk));
        }
    }
}
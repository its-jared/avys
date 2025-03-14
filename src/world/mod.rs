use std::collections::HashMap;

use bevy::prelude::*;
use build::*;

pub mod build;

pub const CHUNK_SIZE: i32 = 100;
pub const CHUNK_HEIGHT: i32 = 100;

pub struct WorldPlugin; 
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkStore(HashMap::default()));
        app.add_systems(Startup, build_world);
        app.add_systems(Update, manage_chunks);
    }
}

#[derive(Resource)]
pub struct ChunkStore(HashMap<IVec2, Handle<Mesh>>);

#[derive(Component)]
pub struct Chunk;

pub struct SpawnChunk(IVec2);
impl Command for SpawnChunk {
    fn apply(self, world: &mut World) {
        if world
            .get_resource_mut::<ChunkStore>()
            .expect("ChunkStore needs to be available.")
            .0
            .get(&self.0)
            .is_some() {
                warn!("Chunk {} already exists.", self.0);
                return;
            };
        
        let chunk_data = build_chunk_data();
        let chunk_mesh = build_chunk_mesh(chunk_data, self.0);

        let mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("Meshes db needs to be available.")
            .add(chunk_mesh);
        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("Materials db needs to be available.")
            .add(Color::WHITE);
        
        world
            .get_resource_mut::<ChunkStore>()
            .expect("ChunkStore needs to be available.")
            .0
            .insert(self.0, mesh.clone());

        world.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(
                self.0.x as f32 * CHUNK_SIZE as f32,
                0.,
                self.0.y as f32 * CHUNK_SIZE as f32
            ),
            Chunk
        ));
    }
}
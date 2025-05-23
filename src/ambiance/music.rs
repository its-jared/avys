use bevy::prelude::*;

#[derive(Component)]
pub struct MusicPlayer;

pub fn setup_music(
    mut c: Commands,
    a: Res<AssetServer>,
) {
    c.spawn((
        AudioPlayer::new(a.load("sounds/music/deep_in_the_moss_garden.wav")),
        MusicPlayer,
    ));
}
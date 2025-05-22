use bevy::prelude::*;
use rand::prelude::*;

const PATH: &str = "sounds/ambiance/forest_";
const NUMBER_OF_SOUNDS: usize = 3;

#[derive(Component)]
pub struct BirdAmbianceNoise;

fn get_random_sound() -> String {
    let mut rng = rand::rng();
    let index = rng.random_range(1..NUMBER_OF_SOUNDS + 1);
    format!("{}{}.wav", PATH, index)
}

pub fn setup_bird_sound(
    mut c: Commands,
    a: Res<AssetServer>
) {
    let actual_path = get_random_sound();
    let audio = a.load(&actual_path);

    info!("Playing: [{}]", actual_path);

    c.spawn((
        AudioPlayer::new(audio),
        BirdAmbianceNoise,
    ));
}

pub fn manage_bird_sound(
    mut q_music: Query<(&mut AudioSink, &mut AudioPlayer), With<BirdAmbianceNoise>>,
    a: Res<AssetServer>,
) {
    if let Ok(mut audio_player) = q_music.single_mut() {
        if audio_player.0.empty() {
            let actual_path = get_random_sound();

            audio_player.1.0 = a.load(&actual_path);
            audio_player.0.play();

            // until i find a way for this to work,
            // this is going to remain commented out
            // since it gets annoying.
            // info!("Playing: [{}]", actual_path);
        }
    }
}
use bevy::prelude::*;

use super::{Health, Mood, MoodComponent};

pub struct PersonalityManagerPlugin;
impl Plugin for PersonalityManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, manage_personalities);
    }
}

fn manage_personalities(
    mut personality_q: Query<(&mut MoodComponent, &Health)>
) {
    for mut personality in personality_q.iter_mut() {
        if personality.1.val == 0.0 {
            personality.0.0 = Mood::Sad;
        }
        else {
            personality.0.0 = Mood::Happy;
        }
    } 
}
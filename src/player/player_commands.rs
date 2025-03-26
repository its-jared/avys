use bevy::ecs::{query::QueryState, system::Commands, world::{Command, World}};
use crate::personality::{Health, Mood, MoodComponent};
use super::Player;

pub struct DamagePlayer {
    pub amount: f32,
}

impl Command for DamagePlayer {
    fn apply(self, world: &mut World) {
        let mut player_q: QueryState<(&Player, &mut Health, &mut MoodComponent)> = world.query();
        let mut player = player_q.single_mut(world);

        player.1.val -= self.amount;

        if player.1.val < 0.0 {
            player.1.val = 0.0;
        }
    }
}

pub struct ChangePlayerMood {
    pub mood: Mood,
}

impl Command for ChangePlayerMood {
    fn apply(self, world: &mut World) {
        let mut player_q: QueryState<(&Player, &mut MoodComponent)> = world.query();
        let mut player = player_q.single_mut(world);

        player.1.0 = self.mood;
    }
}

pub trait PlayerCommands {
    fn damage_player(&mut self, amount: f32);
    fn change_player_mood(&mut self, mood: Mood);
}

impl<'w, 's> PlayerCommands for Commands<'w, 's> { 
    fn damage_player(&mut self, amount: f32) {
        self.queue(DamagePlayer {
            amount: amount
        });
    }

    fn change_player_mood(&mut self, mood: Mood) {
        self.queue(ChangePlayerMood {
            mood: mood
        });
    }
}
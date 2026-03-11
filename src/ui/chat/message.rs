use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub text: String, 
    pub color: Color,
    pub timer: Timer, 
}

impl ChatMessage {
    pub fn new(text: impl Into<String>, color: Color, duration: f32) -> Self {
        Self {
            text: text.into(),
            color, 
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

#[derive(Resource, Default)]
pub struct ChatLog {
    pub messages: VecDeque<ChatMessage>,
    pub max_messages: usize,
}

impl ChatLog {
    pub fn new(max: usize) -> Self {
        Self { messages: VecDeque::new(), max_messages: max }
    }

    pub fn push(&mut self, text: impl Into<String>, color: Color) {
        if self.messages.len() >= self.max_messages {
            self.messages.pop_front();
        }
        self.messages.push_back(ChatMessage::new(text, color, 5.0));
    }
}

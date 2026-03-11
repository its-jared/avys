use bevy::prelude::*;

use crate::ui::chat::message::ChatLog;

pub mod message;

#[derive(Message)]
pub struct PlayerDiedEvent {
    pub player_name: String,
}

#[derive(Message)]
pub struct GameMessageEvent {
    pub text: String,
    pub color: Color,
}

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ChatLog::new(8))
            .add_message::<PlayerDiedEvent>()
            .add_message::<GameMessageEvent>()
            .add_systems(Startup, setup_chat_ui)
            .add_systems(Update, (
                handle_player_died,
                handle_game_messages,
                tick_messages,
                render_chat,
            ).chain());
    }
}


pub fn handle_player_died(
    mut events: MessageReader<PlayerDiedEvent>,
    mut chat: ResMut<ChatLog>,
) {
    for event in events.read() {
        chat.push(
            format!("{} has died!", event.player_name),
            Color::srgb(1.0, 0.2, 0.2),
        );
    }
}

pub fn handle_game_messages(
    mut events: MessageReader<GameMessageEvent>,
    mut chat: ResMut<ChatLog>,
) {
    for event in events.read() {
        chat.push(event.text.clone(), event.color);
    }
}

pub fn tick_messages(
    mut chat: ResMut<ChatLog>,
    time: Res<Time>,
) {
    for msg in chat.messages.iter_mut() {
        msg.timer.tick(time.delta());
    }
    chat.messages.retain(|m| !m.timer.is_finished());
}

#[derive(Component)]
pub struct ChatContainer;

pub fn setup_chat_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        ChatContainer,
    ));
}

pub fn render_chat(
    mut commands: Commands,
    chat: Res<ChatLog>,
    container_query: Query<Entity, With<ChatContainer>>,
) {
    let Ok(container) = container_query.single() else { return };

    commands.entity(container).despawn_children();

    commands.entity(container).with_children(|parent| {
        for msg in chat.messages.iter() {
            let alpha = msg.timer.fraction_remaining();
            let color = msg.color.with_alpha(alpha);

            parent.spawn((
                Text::new(msg.text.clone()),
                TextColor(color),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
            ));
        }
    });
}
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::mem;
use bevy::{
    app::{Plugin, PostUpdate, Update},
    input::ButtonInput,
    prelude::*,
    tasks::{IoTaskPool, block_on, Task, futures_lite::{future}},
};
use bevy::app::Startup;
use bevy::asset::AssetContainer;
use bevy::ecs::bundle::DynamicBundle;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::log::{info};
use bevy::text::Text2dBounds;
use bevy_mod_billboard::{BillboardDepth, BillboardTextBundle};
use bevy_mod_billboard::prelude::BillboardTextBounds;
use bevy_tokio_tasks::TokioTasksRuntime;
use serde_json::Error;
use serde_json_any_key::MapIterToJson;
use tokio::task::JoinHandle;
use crate::character::CharacterTrait;
use crate::communication::{ChatMessage, ChatResponse, Communicator, MessageRole};
use crate::Interaction;
use crate::npc::npc::Npc;
use crate::player::player::Player;

pub struct ActionsPlugin;

// Scales all the 'text-bubbles' spawned above the characters
const TEXT_SCALE: Vec3 = Vec3::splat(0.0030);

// Keep track of remaining text that has yet to be displayed above characters
#[derive(Resource)]
struct ChatBubble {
    queue: HashMap<String, VecDeque<String>>,
}

// To let systems know if the player is currently typing
// Current listeners:
// movement_plugin::update_players_movement_constructor()
#[derive(Event)]
pub struct ToggleInputEvent {
    pub is_toggled: bool,
}

// Event that is emitted when the player sends a message (pressed enter)
// The make_ai_request function picks up on this event and adds it to a HashMap with an id and a JoinHandle (async tokio runtime)
// The get_ai_response functions checks these handles for their status and if they are done handles the model's response
#[derive(Event)]
pub struct AiRequestEvent {
    pub msg: String,
}

// Keeps track of the AI responses tied to the character ID that requested a response
#[derive(Resource)]
struct AiRequestTask {
    generated_response: HashMap<String, JoinHandle<Interaction>>,
}

// Component added to TextBundles so we can make them disappear after a while
#[derive(Component, Eq, PartialEq)]
struct Bubble {
    id: String,
    timer: Timer,
}

// Allows for the listen_keyboard_input_events function to have a local state
#[derive(Default)]
struct SystemInputState {
    is_typing: bool,
}

// Initialize our resources
fn create_resource(mut commands: Commands) {
    commands.insert_resource(AiRequestTask { generated_response: HashMap::new() });
    commands.insert_resource(ChatBubble { queue: HashMap::new() });
}

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToggleInputEvent>();
        app.add_event::<AiRequestEvent>();
        app.add_systems(Startup, (create_resource, setup_scene));
        app.add_systems(Update, (make_bubbles_follow_entities));
        app.add_systems(PostUpdate, (make_ai_request, listen_keyboard_input_events).run_if(resource_exists::<AiRequestTask>));
        app.add_systems(PostUpdate, (get_ai_response.run_if(resource_exists::<AiRequestTask>), bubbling_text));
    }
}

// Listen for AI requests and create async runtime functions to wait for responses
fn make_ai_request(
    mut player_query: Query<&mut Player>,
    mut npc_query: Query<&mut Npc>,
    mut my_tasks: ResMut<AiRequestTask>,
    runtime: ResMut<TokioTasksRuntime>,
    mut on_ai_request: EventReader<AiRequestEvent>,
) {
    if on_ai_request.is_empty() {
        return;
    }

    for req in on_ai_request.read() {
        for mut player in player_query.iter_mut() {
            for mut npc in npc_query.iter_mut() {
                let message = req.msg.clone();
                npc.message_history.push(ChatMessage::new(MessageRole::User, message.clone()));
                let mut npc_clone = npc.clone();
                let player_clone = player.clone();
                let p_name = player.name.clone();
                let n_name = npc.name.clone();

                let task = runtime.spawn_background_task(|mut ctx| async move {
                    let p_name = player_clone.name.clone();
                    let n_name = npc_clone.name.clone();
                    let it = Interaction {
                        sender_id: p_name,
                        receiver_id: n_name,
                        message,
                        actions: vec![],
                    };

                    let mut js = serde_json::to_string(&it).unwrap();
                    js.push_str(npc_clone.get_items().to_json_map().unwrap().as_str());

                    let cm = ChatMessage::new(MessageRole::User, js);
                    let t = npc_clone.talk(cm).await;
                    serde_json::from_str::<Interaction>(t.as_str()).unwrap_or(Interaction {
                        sender_id: "error".to_string(),
                        receiver_id: "error".to_string(),
                        message: "There has been an error converting the AI response (actions_plugin::make_ai_request)".to_string(),
                        actions: vec![],
                    })
                });
                my_tasks.generated_response.insert(format!("{}-{}", p_name, n_name), task);
            }
        }
    }
}

// Checks if the AI responses are done and if so; handles them
fn get_ai_response(mut commands: Commands, mut player_query: Query<&mut Player>, mut npc_query: Query<(&mut Npc, &Transform)>, mut my_tasks: ResMut<AiRequestTask>, mut bubble_queue: ResMut<ChatBubble>) {
    my_tasks.generated_response.retain(|id, task| {
        let status = block_on(future::poll_once(task));

        let retain = status.is_none();

        if let Some(mut res) = status {
            if let Ok(content) = res {
                info!("{}", content.message);
                for player in player_query.iter_mut() {
                    for (mut npc, npc_transform) in npc_query.iter_mut() {
                        if npc.name == content.sender_id {
                            npc.message_history.push(ChatMessage::new(MessageRole::Assistant, content.message.clone()));
                            // Determine how many text sections we need depending on the message length;
                            let sections = max(1, (content.message.len()) / 200);
                            println!("{}", sections);
                            let section_size = content.message.len() / sections;
                            for i in 1..sections {
                                let mut section = String::new();
                                if i == sections - 1 {
                                    section = content.message[i * section_size..].to_string();
                                } else {
                                    section = format!("{}{}", content.message[i * section_size..(i + 1) * section_size].to_string(), "...");
                                }
                                println!("{}", section);
                                bubble_queue.queue.entry(npc.name.clone()).or_insert(VecDeque::from(vec![section.to_string()])).push_back(section.to_string());
                            }
                            commands.spawn(
                                (BillboardTextBundle {
                                    transform: Transform::from_xyz(npc_transform.translation.x, npc_transform.translation.y + 1.5, npc_transform.translation.z).with_scale(TEXT_SCALE),
                                    text: Text::from_section(
                                        &content.message[..section_size].to_string(),
                                        TextStyle {
                                            font_size: 60.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    )
                                        .with_justify(JustifyText::Center),
                                    text_bounds: BillboardTextBounds(Text2dBounds { size: Vec2::new(1200., 500.) }),
                                    billboard_depth: BillboardDepth(false),
                                    ..default()
                                }, Bubble { id: npc.name.clone(), timer: Timer::from_seconds(7., TimerMode::Once) }));
                        }
                    }
                }
            }
        }
        retain
    });
}

// Bottom left input text so the player sees what they type
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // The default font has a limited number of glyphs, so use the full version for
    // sections that will hold text input.
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn(TextBundle {
        text: Text::from_section(
            "".to_string(),
            TextStyle {
                font,
                font_size: 20.0,
                ..default()
            },
        ),
        ..default()
    }.with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }));
}

// Checks for text bubbles that should be de-spawned
// Checks if the next section of text should be pushed if the dialogue is big
fn bubbling_text(
    mut commands: Commands,
    mut bubbles: Query<(Entity, &mut Transform, &mut Bubble)>,
    time: Res<Time>,
    mut bubble_queue: ResMut<ChatBubble>,
) {
    for (entity, mut transform, mut bubble) in bubbles.iter_mut() {
        if bubble.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
            if bubble_queue.queue.contains_key(&bubble.id) {
                if bubble_queue.queue[&bubble.id].is_empty() {
                    return;
                }
                commands.spawn(
                    (BillboardTextBundle {
                        transform: transform.clone(),
                        text: Text::from_section(
                            bubble_queue.queue.get_mut(&bubble.id).unwrap().pop_front().unwrap(),
                            TextStyle {
                                font_size: 60.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                            .with_justify(JustifyText::Center),
                        billboard_depth: BillboardDepth(false),
                        text_bounds: BillboardTextBounds(Text2dBounds { size: Vec2::new(1000., 500.) }),
                        ..default()
                    }, Bubble { id: bubble.id.to_string(), timer: Timer::from_seconds(7., TimerMode::Once) }));
            }
        }
    }
}

// Makes sure the text-bubbles stick to the characters when they move
fn make_bubbles_follow_entities(player_query: Query<(&Player, &Transform)>, npc_query: Query<(&Npc, &Transform)>, mut bubble_query: Query<(&mut Transform, &mut Bubble), (Without<Player>, Without<Npc>)>) {
    for (mut bubble_transform, bubble) in bubble_query.iter_mut() {
        let mut transform = bubble_transform.clone();
        if let Some((p, p_t)) = player_query.iter().find(|(p, p_t)| p.name == bubble.id) {
            transform = p_t.clone();
        } else if let Some((n, n_t)) = npc_query.iter().find(|(n, n_t)| n.name == bubble.id) {
            transform = n_t.clone();
        } else {
            continue;
        }
        bubble_transform.translation.x = transform.translation.x;
        bubble_transform.translation.y = transform.translation.y + 1.5;
        bubble_transform.translation.z = transform.translation.z;
    }
}

fn listen_keyboard_input_events(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, (Without<Bubble>)>,
    mut is_typing: Local<SystemInputState>,
    mut emit_ai_request: EventWriter<AiRequestEvent>,
    mut toggle_input_events: EventWriter<ToggleInputEvent>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut npc_query: Query<(&mut Npc, &Transform)>,
) {
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if !event.state.is_pressed() {
            continue;
        }
        match &event.logical_key {
            Key::Enter => {
                if !is_typing.is_typing {
                    continue;
                }
                let mut text = edit_text.single_mut();
                if text.sections[0].value.is_empty() {
                    continue;
                }
                let old_value = mem::take(&mut text.sections[0].value);
                println!("{}", old_value.clone());
                let (player, player_transform) = player_query.single_mut();
                commands.spawn(create_text_bundle(player.name.clone(), old_value.clone(), &Transform::from_xyz(player_transform.translation.x, player_transform.translation.y + 1.5, player_transform.translation.z)));
                emit_ai_request.send(AiRequestEvent { msg: old_value.clone() });
                toggle_input_events.send(ToggleInputEvent { is_toggled: false });
                is_typing.is_typing = false;
            }
            Key::Space => {
                if is_typing.is_typing {
                    edit_text.single_mut().sections[0].value.push(' ');
                } else {
                    for (_, p_transform) in player_query.iter() {
                        for (__, n_transform) in npc_query.iter() {
                            if (p_transform.translation - n_transform.translation).length() < 2.0 {
                                is_typing.is_typing = true;
                                toggle_input_events.send(ToggleInputEvent { is_toggled: true });
                            }
                        }
                    }
                }
            }
            Key::Backspace => {
                if !is_typing.is_typing {
                    continue;
                }
                edit_text.single_mut().sections[0].value.pop();
            }
            Key::Escape => {
                if !is_typing.is_typing {
                    continue;
                }
                edit_text.single_mut().sections[0].value.clear();
                is_typing.is_typing = false;
                toggle_input_events.send(ToggleInputEvent { is_toggled: false });
            }
            Key::Character(character) => {
                if !is_typing.is_typing {
                    continue;
                }
                edit_text.single_mut().sections[0].value.push_str(character);
            }
            _ => continue,
        }
    }
}

fn create_text_bundle(id: String, msg: String, transform: &Transform) -> (BillboardTextBundle, Bubble) {
    (BillboardTextBundle {
        transform: transform.clone().with_scale(TEXT_SCALE),
        text: Text::from_section(
            msg,
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        )
            .with_justify(JustifyText::Center),
        billboard_depth: BillboardDepth(false),
        text_bounds: BillboardTextBounds(Text2dBounds { size: Vec2::new(1000., 500.) }),
        ..default()
    }, Bubble { id, timer: Timer::from_seconds(7., TimerMode::Once) })
}
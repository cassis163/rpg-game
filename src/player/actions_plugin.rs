use std::collections::HashMap;
use std::mem;
use bevy::{
    app::{Plugin, PostUpdate, Update},
    input::ButtonInput,
    prelude::*,
    tasks::{IoTaskPool, block_on, Task, futures_lite::{future}},
};
use bevy::app::Startup;
use bevy::asset::AssetContainer;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::log::{info};
use bevy_tokio_tasks::TokioTasksRuntime;
use serde_json::Error;
use serde_json_any_key::MapIterToJson;
use tokio::task::JoinHandle;
use crate::character::CharacterTrait;
use crate::communication::{ChatMessage, ChatResponse, Communicator, MessageRole};
use crate::Interaction;
use crate::npc::npc::Npc;
use crate::player::player::Player;

pub struct ActionsPlugin {
    pub(crate) blocking: bool,
}

fn should_run(content: Option<Res<AiRequestTask>>) -> bool {
    content.is_some()
}

fn create_resource(mut commands: Commands) {
    commands.insert_resource(AiRequestTask { generated_response: HashMap::new() })
}

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToggleInputEvent>();
        app.add_event::<AiRequestEvent>();
        app.add_systems(Startup, (create_resource, setup_scene));
        app.add_systems(Update, listen_interaction);
        app.add_systems(PostUpdate, (make_ai_request, listen_keyboard_input_events).run_if(resource_exists::<AiRequestTask>));
        app.add_systems(PostUpdate, (get_ai_response.run_if(resource_exists::<AiRequestTask>), bubbling_text));
    }
}

#[derive(Event)]
pub struct ToggleInputEvent {
    pub is_toggled: bool,
}

#[derive(Event)]
pub struct AiRequestEvent {
    pub msg: String,
}

#[derive(Resource)]
struct AiRequestTask {
    generated_response: HashMap<String, JoinHandle<Interaction>>,
}

fn listen_interaction(
    mut player_query: Query<(&mut Player, &Transform)>,
    mut npc_query: Query<(&mut Npc, &Transform)>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_input_events: EventWriter<ToggleInputEvent>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        for (_, p_transform) in player_query.iter() {
            for (__, n_transform) in npc_query.iter() {
                if (p_transform.translation - n_transform.translation).length() < 2.0 {
                    info!("Was in range!");
                    toggle_input_events.send(ToggleInputEvent { is_toggled: true });
                }
            }
        }
    } else if key_input.just_pressed(KeyCode::Enter) {
        toggle_input_events.send(ToggleInputEvent { is_toggled: false });
    }
}

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
                        message: "error".to_string(),
                        actions: vec![],
                    })
                });
                my_tasks.generated_response.insert(format!("{}-{}", p_name, n_name), task);
            }
        }
    }
}

fn get_ai_response(mut commands: Commands, mut player_query: Query<&mut Player>, mut npc_query: Query<&mut Npc>, mut my_tasks: ResMut<AiRequestTask>) {
    my_tasks.generated_response.retain(|id, task| {
        let status = block_on(future::poll_once(task));

        let retain = status.is_none();

        if let Some(mut res) = status {
            if let Ok(content) = res {
                info!("{}", content.message);
                for player in player_query.iter_mut() {
                    for mut npc in npc_query.iter_mut() {
                        if npc.name == content.sender_id {
                            npc.message_history.push(ChatMessage::new(MessageRole::Assistant, content.message.clone()));
                            println!("{}", npc.message_history.len());
                            commands.spawn((
                                TextBundle {
                                    text: Text::from_section(content.message.clone(), Default::default()),
                                    ..default()
                                }.with_text_justify(JustifyText::Center).with_style(Style {
                                    position_type: PositionType::Absolute,
                                    bottom: Val::Px(5.0),
                                    right: Val::Px(5.0),
                                    ..default()
                                }),
                                Bubble {
                                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                                },
                            ));
                        }
                    }
                }
            }
        }
        retain
    });
}

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


#[derive(Component)]
struct Bubble {
    timer: Timer,
}

fn bubbling_text(
    mut commands: Commands,
    mut bubbles: Query<(Entity, &mut Transform, &mut Bubble)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bubble) in bubbles.iter_mut() {
        if bubble.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
        transform.translation.y += time.delta_seconds() * 100.0;
    }
}

#[derive(Default)]
struct SystemInputState {
    is_typing: bool,
}
fn listen_keyboard_input_events(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, (Without<Bubble>)>,
    mut toggle_input_event: EventReader<ToggleInputEvent>,
    mut is_typing: Local<SystemInputState>,
    mut emit_ai_request: EventWriter<AiRequestEvent>
) {
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if !event.state.is_pressed() {
            continue;
        }
        let is_talking = toggle_input_event.read().any(|tie| {
            tie.is_toggled
        });
        if !is_talking && !is_typing.is_typing {
            continue;
        }
        is_typing.is_typing = true;

        match &event.logical_key {
            Key::Enter => {
                let mut text = edit_text.single_mut();
                if text.sections[0].value.is_empty() {
                    continue;
                }
                let old_value = mem::take(&mut text.sections[0].value);
                println!("{}", old_value.clone());

                commands.spawn((
                    TextBundle {
                        text: Text::from_section(old_value.clone(), text.sections[0].style.clone()),
                        ..default()
                    },
                    Bubble {
                        timer: Timer::from_seconds(5.0, TimerMode::Once),
                    },
                ));
                emit_ai_request.send(AiRequestEvent{msg: old_value.clone()});
                is_typing.is_typing = false;
            }
            Key::Space => {
                edit_text.single_mut().sections[0].value.push(' ');
            }
            Key::Backspace => {
                edit_text.single_mut().sections[0].value.pop();
            }
            Key::Character(character) => {
                edit_text.single_mut().sections[0].value.push_str(character);
            }
            _ => continue,
        }
    }
}
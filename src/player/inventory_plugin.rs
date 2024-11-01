use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use crate::item::{Item, ItemType};
use crate::npc::npc::Npc;
use crate::player::actions_plugin::ToggleInputEvent;

pub struct InventoryPlugin;

#[derive(Component)]
struct Inventory;

#[derive(Component)]
struct DraggableItem;

#[derive(Component)]
struct InventoryItem {
    index: usize,
    name: String,

}

#[derive(Default)]
struct InventoryState {
    can_open: bool,
    is_open: bool,
}

#[derive(Resource)]
struct Dragging {
    is_dragging: bool,
    item_index: Option<usize>,
    item_initial_pos: Option<Vec2>,
}

#[derive(Component)]
struct GiveUI;

#[derive(Component)]
struct DropZone;

#[derive(Event)]
struct ToggleInventoryEvent {
    is_open: bool,
}

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToggleInventoryEvent>();
        app.add_systems(Startup, (add_resources, setup_inventory_ui, setup_give_ui));
        app.add_systems(Update, (toggle_inventory_constructor(), click_item, drag_item, toggle_give_ui));
    }
}

fn add_resources(mut commands: Commands) {
    commands.insert_resource(Dragging { is_dragging: false, item_index: None, item_initial_pos: None });
}

fn setup_give_ui(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let dropzone_size = Vec2::new(window.height() * 0.6 / 5., window.height() * 0.6 / 5.);
    commands.spawn((GiveUI, NodeBundle {
        style: Style {
            display: Display::Block,
            left: Val::Px(window.width() / 2.),
            ..default()
        },
        ..default()
    })).with_children(|parent| {
        parent.spawn((
            DropZone,
            NodeBundle { style: Style { width: Val::Px(dropzone_size.x), height: Val::Px(dropzone_size.y), ..default() }, background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)), ..default() }
        ));
    });
}

fn setup_inventory_ui(mut commands: Commands) {
    let mut items = HashMap::new();
    items.insert(Item::new("Steel Sword".to_string(), ItemType::Weapon, "A sturdy steel sword".to_string(), ("Gold Coin".to_string(), 5)), 1);
    items.insert(Item::new("Gold Coin".to_string(), ItemType::Currency, "A flat and round piece of gold used to buy goods".to_string(), ("Gold Coin".to_string(), 1)), 1);
    // Spawn the inventory grid
    commands.spawn((Inventory, NodeBundle {
        visibility: Visibility::Hidden,
        style: Style {
            display: Display::Grid,
            // Set up a grid with 5 columns and rows
            grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(5, 1.0),
            height: Val::Percent(60.),
            aspect_ratio: Some(1.),
            ..default()
        },
        background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
        ..default()
    }))
        .with_children(|parent| {
            for i in 1..=items.len() {
                let name = items.keys().nth(i - 1).unwrap().name.clone();
                parent.spawn((
                    DraggableItem,
                    InventoryItem { index: i - 1, name: name.clone() },
                    TextBundle {
                        text: Text::from_section(name.clone(), Default::default()),
                        style: Style {
                            //margin: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba((i as f32) / 10., 1. / (i as f32), 1. / (i as f32), 1.0)),
                        z_index: ZIndex::Global(1),
                        ..default()
                    },
                ));
            }
        });
}

fn click_item(
    inventory_query: Query<&Visibility, With<Inventory>>,
    mut inventory_item_query: Query<(&GlobalTransform, &mut Style, &InventoryItem)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut dragging: ResMut<Dragging>,
) {
    if *inventory_query.single() == Visibility::Hidden {
        return;
    }
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = window_query.single();
        if let Some(cursor_pos) = window.cursor_position() {
            for (item_transform, _style, item) in inventory_item_query.iter_mut() {
                let item_size = Vec2::new(window.height() * 0.6 / 5., window.height() * 0.6 / 5.);
                let item_pos = Vec2::new(item_transform.translation().x, item_transform.translation().y);

                if cursor_pos.x >= (item_pos.x - item_size.x / 2.) && cursor_pos.x <= (item_pos.x + item_size.x / 2.) {
                    if cursor_pos.y >= (item_pos.y - item_size.y / 2.) && cursor_pos.y <= (item_pos.y + item_size.y / 2.) {
                        println!("Clicked on: {}", item.name);
                        dragging.is_dragging = true;
                        dragging.item_index = Some(item.index);
                        dragging.item_initial_pos = Some(item_pos);
                    }
                }
            }
        }
    } else if mouse_input.just_released(MouseButton::Left) {
        for (_item_transform, mut style, item) in inventory_item_query.iter_mut() {
            if let (Some(d), Some(_i)) = (dragging.item_index, dragging.item_initial_pos) {
                if d == item.index {
                    style.left = Val::Auto;
                    style.top = Val::Auto;
                }
            }
        }
        dragging.is_dragging = false;
        dragging.item_index = None;
    }
}

fn drag_item(mut inventory_item_query: Query<(&mut Style, &mut ZIndex, &InventoryItem)>, _window_query: Query<&Window, With<PrimaryWindow>>, mut cursor_moved: EventReader<CursorMoved>, dragging: Res<Dragging>) {
    if !dragging.is_dragging {
        return;
    }
    if let Some(cm) = cursor_moved.read().last() {
        if let (Some(item_index), Some(initial_pos)) = (dragging.item_index, dragging.item_initial_pos) {
            for (mut item_style, mut item_z_index, item) in inventory_item_query.iter_mut() {
                if item.index == item_index {
                    item_style.left = Val::Px(cm.position.x - initial_pos.x);
                    item_style.top = Val::Px(cm.position.y - initial_pos.y);
                    *item_z_index = ZIndex::Global(2);
                }
            }
        }
    }
}


fn toggle_inventory_constructor() -> impl FnMut(Query<&mut Visibility, With<Inventory>>, EventReader<KeyboardInput>, EventReader<ToggleInputEvent>, EventWriter<ToggleInventoryEvent>) {
    let mut inventory_state = InventoryState { can_open: true, is_open: false };
    move |mut inventory_query, mut input_events, mut can_input, mut toggle_inventory_event: EventWriter<ToggleInventoryEvent>| {
        // We can open the inventory if we are not typing in chat
        // See: actions_plugin
        for tie in can_input.read() {
            inventory_state.can_open = !tie.is_toggled;
        }

        if !inventory_state.can_open {
            return;
        }

        for events in input_events.read() {
            if events.state.is_pressed() {
                continue;
            }

            match events.key_code {
                KeyCode::KeyI => {
                    if inventory_state.can_open {
                        inventory_state.is_open = !inventory_state.is_open;
                        *inventory_query.single_mut() = if inventory_state.is_open { Visibility::Visible } else { Visibility::Hidden };
                        toggle_inventory_event.send(ToggleInventoryEvent { is_open: inventory_state.is_open });
                    }
                }
                _ => {}
            }
        }
    }
}

fn toggle_give_ui(
    mut give_ui_query: Query<(&mut Visibility, &mut Style, &GlobalTransform), With<GiveUI>>,
    npc_query: Query<&GlobalTransform, With<Npc>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut on_inventory_toggle: EventReader<ToggleInventoryEvent>,
    window: Query<&Window>,
) {
    if let Some(tie) = on_inventory_toggle.read().last() {
        for npc_transform in npc_query.iter() {
            for (camera, camera_transform) in camera_query.iter() {
                if let Some(vp_coords) = camera.world_to_viewport(camera_transform, npc_transform.translation()) {
                    for (mut visibility, mut style, _transform) in give_ui_query.iter_mut() {
                        println!("{:?}", vp_coords);
                        *visibility = if tie.is_open { Visibility::Visible } else { Visibility::Hidden };
                        if tie.is_open {
                            style.left = Val::Px(vp_coords.x);
                            style.top = Val::Px(vp_coords.y);
                        } else {
                            style.left = Val::Px(window.single().width() / 2.);
                            style.top = Val::Px(window.single().height() / 2.);
                        }
                    }
                }
            }
        }
    }
}

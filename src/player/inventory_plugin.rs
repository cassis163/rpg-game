use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::color::palettes::css::*;
use crate::item::{Item, ItemType};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_inventory_ui);
    }
}

fn setup_inventory_ui(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
    let mut items = vec![
        Item::new("Steel Sword".to_string(), ItemType::Weapon, "A simple but trustworthy weapon".to_string(), ("Gold Coin".to_string(), 20))
    ];
    let inventory_size = 25;

    // Inventory Panel
    commands.spawn(NodeBundle {
        style: Style {
            // Make the height of the node fill its parent
            height: Val::Percent(60.0),
            // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
            // As the height is set explicitly, this means the width will adjust to match the height
            aspect_ratio: Some(1.0),
            // Use grid layout for this node
            display: Display::Grid,
            // Add 24px of padding around the grid
            padding: UiRect::all(Val::Px(24.0)),
            // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
            // This creates 4 exactly evenly sized columns
            grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
            // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
            // This creates 4 exactly evenly sized rows
            grid_template_rows: RepeatedGridTrack::flex(5, 1.0),
            // Set a 12px gap/gutter between rows and columns
            row_gap: Val::Px(12.0),
            column_gap: Val::Px(12.0),
            ..default()
        },
        background_color: BackgroundColor(Color::srgba(0.25, 0.25, 0.25, 0.6)),
        ..default()
    })
        .with_children(|builder| {
            // Note there is no need to specify the position for each grid item. Grid items that are
            // not given an explicit position will be automatically positioned into the next available
            // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
            // style property.

            for i in 0..inventory_size {
                if i >= items.len() {
                    item_rect(builder, GRAY);
                    continue;
                }
                item_rect(builder, BLUE);
            }
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take its size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Srgba) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(BLACK.into()),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color.into()),
                ..default()
            });
        });
}

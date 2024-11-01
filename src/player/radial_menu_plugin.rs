use std::f32::consts::PI;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

pub struct RadialMenuPlugin;

impl Plugin for RadialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_radial_menu, create_resources.before(setup_radial_menu)));
        app.add_systems(Update, check_mouse_hover);
    }
}

#[derive(Component)]
struct Sector {
    action: String,
}


#[derive(Resource)]
struct RadialMenuState {
    num_sectors: i8,
}

fn create_resources(
    mut commands: Commands,
) {
    commands.insert_resource(RadialMenuState{num_sectors: 4});
}


fn setup_radial_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut radial_menu_state: ResMut<RadialMenuState>,
) {
    let mut ui_cam = Camera2dBundle::default();
    // Make sure the UI camera is the secondary camera and doesn't clear the 3D camera
    ui_cam.camera.order = 2;
    ui_cam.camera.clear_color = ClearColorConfig::None;
    commands.spawn(ui_cam);

    let options = ["Talk", "Talk to", "Give", "Inventory"];
    //let options = ["Talk", "Talk to", "Give"];
    //let options = ["Talk", "Talk to"];
    //let options = ["Talk"];

    radial_menu_state.num_sectors = options.len() as i8;

    let radius = 150.;

    for (i, option) in options.iter().enumerate() {
        let mut sector = Transform::from_xyz(0., 0., 1.);
        let sector_rotation = get_rotation(i, radial_menu_state.num_sectors as f32);
        sector.rotate_z(sector_rotation);

        // Radial menu shape
        commands.spawn((Sector { action: option.to_string() }, MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(CircularSector::new(radius,  (PI) / radial_menu_state.num_sectors as f32))),
            material: materials.add(Color::srgba(0.2, 0.2, 0.2, 0.4)),
            transform: sector,
            ..default()
        }));

        let (mut start_angle, mut mid_angle, _end_angle) = get_sector(i, radial_menu_state.num_sectors as f32);

        // Line seperator
        let (start, end) = (Vec2::new(0., 0.), Vec2::new(radius * start_angle.cos(), radius * start_angle.sin()));
        // Create a mesh for the line
        let direction = (end - start).normalize();
        let length = start.distance(end);

        // Create a transform to place the rectangle (quad) properly
        let angle = direction.y.atan2(direction.x);
        let mid_point = (start + end) / 2.0;

        let thickness = 2.0;

        let transform = Transform {
            translation: Vec3::new(mid_point.x, mid_point.y, 0.0),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec3::new(length, thickness, 1.0),
            ..Default::default()
        };

        // Create a quad mesh (a simple rectangle)
        let mesh = Mesh::from(Rectangle::default());

        let line_color = Color::srgba(1., 1., 1., 0.4);

        // Spawn the line as a quad (rectangle)
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                transform,
                material: materials.add(line_color),
                ..Default::default()
            }
        ));

        // Convert polar coordinates to Cartesian for the center point
        let sector_center = Vec2::new(mid_angle.cos() * radius * 0.5, mid_angle.sin() * radius * 0.5); // 0.6 to move inward slightly
        commands.spawn(Text2dBundle {
            text: Text::from_section(option.to_string(), TextStyle {
                ..default()
            }),
            transform: Transform::from_xyz(sector_center.x, sector_center.y, 2.0),
            ..default()
        });
    }
}

fn get_rotation(i: usize, num_sectors: f32) -> f32 {
    let angle_per_sector = 2. * PI / num_sectors;
    let mut start_angle = -(i as f32 * angle_per_sector);

    // Normalize angles to [0, 2*PI] range
    if start_angle < 0.0 {
        start_angle += 2.0 * PI;
    }

    start_angle
}

fn get_sector(i: usize, num_sectors: f32) -> (f32, f32, f32) {
    let angle_per_sector = 2. * PI / num_sectors;
    let mut start_angle = -(i as f32 * angle_per_sector - PI / 4.);
    let mut mid_angle = start_angle + (angle_per_sector / 2.);
    let mut end_angle = start_angle + angle_per_sector;

    // Normalize angles to [0, 2*PI] range
    if start_angle < 0.0 {
        start_angle += 2.0 * PI;
    }
    if end_angle < 0.0 {
        end_angle += 2.0 * PI;
    }

    (start_angle, mid_angle, end_angle)
}

fn check_mouse_hover(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(Entity, &Sector)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    radial_menu_state: Res<RadialMenuState>,
) {
    let window = windows.single();
    if let Some(mouse_position) = window.cursor_position() {
        // Get mouse position relative to the center of the circle
        let mouse_x = mouse_position.x - window.width() / 2.0;
        let mouse_y = mouse_position.y - window.height() / 2.0;

        // Convert to polar coordinates (r and theta)
        let mouse_radius = (mouse_x.powi(2) + mouse_y.powi(2)).sqrt();
        let mut mouse_angle = mouse_y.atan2(mouse_x); // In radians


        // Normalize mouse_angle to [0, 2*PI]
        if mouse_angle < 0.0 {
            mouse_angle += 2.0 * PI;
        }

        // We want it to go clockwise for my sanity's sake :D
        mouse_angle = 2. * PI - mouse_angle;

        // Check if mouse is hovering over a sector
        let sector_radius = 150.0; // Radius of the sectors
        let num_sectors = radial_menu_state.num_sectors;

        for (i, (entity, sector)) in query.iter().enumerate() {
            let (start_angle, _mid_angle, end_angle) = get_sector(i, num_sectors as f32);
            if mouse_radius < sector_radius && is_angle_within_sector(mouse_angle, start_angle, end_angle) {
                println!("Mouse is hovering over sector {}", sector.action);
                commands.entity(entity).insert(materials.add(Color::srgba(1.0, 0.0, 0.0, 0.4)));
            } else {
                commands.entity(entity).insert(materials.add(Color::srgba(0.2, 0.2, 0.2, 0.4)));
            }
        }
    }
}

// Helper function to check if mouse angle is within the sector's range
fn is_angle_within_sector(mouse_angle: f32, start_angle: f32, end_angle: f32) -> bool {
    // Handle cases where the end angle has wrapped around past 2*PI
    if start_angle > end_angle {
        mouse_angle >= start_angle || mouse_angle <= end_angle
    } else {
        mouse_angle >= start_angle && mouse_angle <= end_angle
    }
}

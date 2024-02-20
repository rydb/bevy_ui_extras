use std::f32::consts::PI;

use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ui_extras::{resources::WindowStyleFrame, systems::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_systems(Startup, spawn_world)
        .insert_resource(WindowStyleFrame::default())
        .add_systems(
            Update,
            (
                visualize_right_sidepanel_for::<Handle<Mesh>>,
                visualize_window_for::<Transform>,
            ),
        )
        .run();
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -PI / 4.0,
            0.0,
            0.0,
        )),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            -PI / 4.0,
            0.0,
        )),
        material: materials.add(Color::CYAN),
        ..default()
    });
}

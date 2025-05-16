use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use bevy_ui_extras::{widgets::debug_menu::{plugins::DebugModeFlagRegister, DebugMenuState}, *};
use strum_macros::{Display, EnumIter};


#[derive(Default, Deref, DerefMut, Resource)]
pub struct DummyDebugToggle(pub bool);

fn main() {
    App::new()
        //.init_resource::<QuickTable<MeshAttributes>>()d
        .add_plugins(DefaultPlugins)
        .init_resource::<DummyDebugToggle>()
        .add_plugins(DebugModeFlagRegister::<DummyDebugToggle>::default())
        .register_type::<MeshInfoTarget>()
        .add_plugins(UiExtrasDebug(
            UiExtrasDebugSetup {
                ui_style: UiStyle::BLACK_GLASS,
                alignment: None,
                menu_mode: DebugMenuState::Explain,
                ..default()
            }
        ))
        .add_systems(Startup, spawn_world)
        .add_systems(
            Update,
            (visualize_components_for::<Transform>(
                bevy_ui_extras::Display::Side(Side::Right),
            ),),
        )
        .run();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MeshInfoTarget(pub u32);

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 5.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -PI / 4.0,
            0.0,
            0.0,
        )),
        Name::new("Camera"),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        MeshInfoTarget(10),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            -PI / 4.0,
            0.0,
        )),
        Name::new("Cube"),
    ));
}

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum MeshAttributes {
    #[default]
    POSITION,
    //INDICE,
    NORMAL,
    UV,
}

/// returns a vec with mutable references to the values of mesh attributes.
/// If the attribute doesn't exist, returns none.
pub fn attr_to_vec(attr_fetch: Option<&mut VertexAttributeValues>) -> Option<Vec<Vec<&mut f32>>> {
    let attr = match attr_fetch {
        Some(attr) => attr,
        None => return None,
    };

    match attr {
        VertexAttributeValues::Float32x3(vec) => {
            let mut return_vec: Vec<Vec<&mut f32>> = Vec::new();

            for i in vec.iter_mut() {
                let x = i.iter_mut().collect::<Vec<_>>();
                return_vec.push(x)
            }
            Some(return_vec)
        }
        VertexAttributeValues::Float32x2(vec) => {
            let mut return_vec: Vec<Vec<&mut f32>> = Vec::new();

            for i in vec.iter_mut() {
                let x = i.iter_mut().collect::<Vec<_>>();
                return_vec.push(x)
            }
            Some(return_vec)
        }
        err => panic!("attribute retrieval not implemented for {:#?}", err),
    }
}

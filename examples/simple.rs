use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use bevy_inspector_egui::egui::Align2;
use bevy_ui_extras::*;
use states::DebugMenuState;
use strum_macros::{Display, EnumIter};

#[derive(Default, Deref, DerefMut, Resource)]
pub struct DummyDebugToggle(pub bool);

fn main() {
    App::new()
        //.init_resource::<QuickTable<MeshAttributes>>()
        .add_plugins(DefaultPlugins)
        .init_resource::<DummyDebugToggle>()
        .add_plugins(DebugModeFlagRegister::<DummyDebugToggle>::default())
        .register_type::<MeshInfoTarget>()
        .add_plugins(UiExtrasDebug {
            ui_style: UiStyle::BLACK_GLASS,
            alignment: UiAlignment(Align2::LEFT_TOP),
            menu_mode: DebugMenuState::Explain,
            ..default()
        })
        .add_systems(Startup, spawn_world)
        .add_systems(
            Update,
            (
                // visualize_entities_with_component::<MeshMaterial3d<StandardMaterial>>(
                //     bevy_ui_extras::Display::Side(Side::Left),
                // ),
                visualize_components_for::<Transform>(bevy_ui_extras::Display::Side(Side::Right)),
                //visualize_resource::<ClearColor>(bevy_ui_extras::Display::Window),
                //display_mesh_info,
            ),
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

/// creates a table displaying info about mesh targets, and a menu to edit these meshesh through.
// pub fn display_mesh_info(
//     mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
//     //mut mesh_attr_table: ResMut<TablePick<MeshAttributes>>,
//     mut mesh_attr_table: ResMut<QuickTable<MeshAttributes>>,
//     target_meshes: Query<&Handle<Mesh>, With<MeshInfoTarget>>,
//     mut meshes: ResMut<Assets<Mesh>>
// ) {
//     for mut context in primary_window.iter_mut() {
//         let ui_name = "Mesh Attributes";
//         egui::Window::new(ui_name)
//         .resizable(false)
//         .show(context.get_mut(), |ui| {
//             for mesh_check in target_meshes.iter() {
//                 let Some(mesh) = meshes.get_mut(mesh_check) else {continue;};

//                 mesh_attr_table.ui(ui)
//                 .body(|mut body| {
//                     body.row(20.0, |mut row| {
//                         for attr_type in MeshAttributes::iter() {
//                              if mesh_attr_table.0.contains_key(&attr_type.to_string()) {
//                                 row.col(|ui| {
//                                     match attr_type {
//                                         MeshAttributes::POSITION => {
//                                             let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)).unwrap_or_default();
//                                                 for vertex in pos_vertices.iter_mut() {
//                                                     ui.horizontal(|ui| {
//                                                         for n in vertex.iter_mut() {
//                                                             ui.add(egui::DragValue::new(*n).speed(0.1));
//                                                         }
//                                                     });
//                                                 }
//                                         },
//                                         // MeshAttributes::INDICE => {
//                                         //     let Some(indicies_type) = mesh.indices() else {return;};
//                                         //     let mut indicies = Vec::new();
//                                         //     match indicies_type {
//                                         //         Indices::U32(vec) => {
//                                         //             for n in vec {
//                                         //                 indicies.push(*n)
//                                         //             }
//                                         //         },
//                                         //         Indices::U16(vec) => {
//                                         //             for n in vec {
//                                         //                 indicies.push(*n as u32)
//                                         //             }
//                                         //         }
//                                         //     };
//                                         //     let grouped = indicies.chunks_exact(3);
//                                         //     for indice in grouped.into_iter() {
//                                         //         ui.horizontal(|ui| {
//                                         //             for n in indice.iter() {
//                                         //                 ui.label(n.to_string());
//                                         //             }
//                                         //         });
//                                         //     }
//                                         // },
//                                         MeshAttributes::NORMAL => {
//                                             let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)).unwrap_or_default();
//                                                 for vertex in pos_vertices.iter_mut() {
//                                                     ui.horizontal(|ui| {
//                                                         for n in vertex.iter_mut() {
//                                                             ui.add(egui::DragValue::new(*n).speed(0.1));
//                                                         }
//                                                     });
//                                                 }
//                                         },
//                                         MeshAttributes::UV => {
//                                             let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)).unwrap_or_default();
//                                                 for vertex in pos_vertices.iter_mut() {
//                                                     ui.horizontal(|ui| {
//                                                         for n in vertex.iter_mut() {
//                                                             ui.add(egui::DragValue::new(*n).speed(0.1));
//                                                         }
//                                                     });
//                                                 }
//                                         },
//                                     }
//                                 });
//                              } else {
//                                 row.col(|ui| {ui.label("");});
//                              }
//                         }
//                     });
//                 });
//             }
//         });
//     }
// }

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

use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use bevy_egui::EguiContext;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ui_extras::{resources::WindowStyleFrame, systems::*, tables::resources::{TablePick, TableTemplate}};
use bevy_window::PrimaryWindow;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() {
    App::new()
    .init_resource::<TablePick<MeshAttributes>>()
    .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
    .add_systems(Startup, spawn_world)
    .insert_resource(WindowStyleFrame::default())
    .add_systems(
        Update,
        (
            //visualize_window_for::<Transform>,
            visualize_right_sidepanel_for::<Transform>,

            //visualize_window_for::<Transform>,
            display_mesh_info,
        ),
    )
    .run();
}

#[derive(Component)]
pub struct MeshInfoTarget;

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
    commands.spawn(
        (
        PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            -PI / 4.0,
            0.0,
        )),
        material: materials.add(Color::hsl(180.0, 1.0, 0.5)),
        ..default()
    },
    MeshInfoTarget
        )
);
}

// #[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
// pub enum MeshAttributes {
//     #[default]
//     POSITION,
//     //INDICE,
//     NORMAL,
//     UV,
// }

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum MeshAttributes {
    #[default]
    POSITION,
    //INDICE,
    NORMAL,
    UV,
}

// pub fn say_hi() {
//     println!("hewooo! OwO")
// }
// pub fn say_bye () {
//     println!("goodbyee ;c;")
// }

// pub fn attribute_windows() {
//     let tabs = HashMap::<

//     tabs.insert("cool", say_hi);

//     tabs.insert("lame", say_bye);
// }


/// creates a table displaying info about mesh targets, and a menu to edit these meshesh through. 
pub fn display_mesh_info(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut mesh_attr_table: ResMut<TablePick<MeshAttributes>>,
    target_meshes: Query<&Handle<Mesh>, With<MeshInfoTarget>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    for mut context in primary_window.iter_mut() {
        let ui_name = "Mesh Attributes";
        egui::Window::new(ui_name)
        .resizable(false)
        .show(context.get_mut(), |ui| {
            for mesh_check in target_meshes.iter() {
                let Some(mesh) = meshes.get_mut(mesh_check) else {continue;};
                
                TableTemplate::new(ui, &mut *mesh_attr_table)
                .body(|mut body| {
                    body.row(20.0, |mut row| {
                        for attr_type in MeshAttributes::iter() {
                             if mesh_attr_table.contains_key(&attr_type.to_string()) {
                                row.col(|ui| {
                                    match attr_type {
                                        MeshAttributes::POSITION => {
                                            let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)).unwrap_or_default();                                            
                                                for vertex in pos_vertices.iter_mut() {
                                                    ui.horizontal(|ui| {
                                                        for n in vertex.iter_mut() {
                                                            ui.add(egui::DragValue::new(*n).speed(0.1));
                                                        }
                                                    });   
                                                }     
                                        },
                                        // MeshAttributes::INDICE => {
                                        //     let Some(indicies_type) = mesh.indices() else {return;};
                                        //     let mut indicies = Vec::new();
                                        //     match indicies_type {
                                        //         Indices::U32(vec) => {
                                        //             for n in vec {
                                        //                 indicies.push(*n)
                                        //             }  
                                        //         },
                                        //         Indices::U16(vec) => {
                                        //             for n in vec {
                                        //                 indicies.push(*n as u32)
                                        //             }  
                                        //         }
                                        //     };
                                        //     let grouped = indicies.chunks_exact(3);
                                        //     for indice in grouped.into_iter() {
                                        //         ui.horizontal(|ui| {
                                        //             for n in indice.iter() {
                                        //                 ui.label(n.to_string());
                                        //             }
                                        //         });
                                        //     }     
                                        // },
                                        MeshAttributes::NORMAL => {
                                            let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)).unwrap_or_default();                                            
                                                for vertex in pos_vertices.iter_mut() {
                                                    ui.horizontal(|ui| {
                                                        for n in vertex.iter_mut() {
                                                            ui.add(egui::DragValue::new(*n).speed(0.1));
                                                        }
                                                    });   
                                                }             
                                        },
                                        MeshAttributes::UV => {
                                            let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)).unwrap_or_default();                                            
                                                for vertex in pos_vertices.iter_mut() {
                                                    ui.horizontal(|ui| {
                                                        for n in vertex.iter_mut() {
                                                            ui.add(egui::DragValue::new(*n).speed(0.1));
                                                        }
                                                    });   
                                                }              
                                        },
                                    }
                                });
                             } else {
                                row.col(|ui| {ui.label("");});
                             }
                        }
                    });
                });
            }
        });
    }
}

/// returns a vec with mutable references to the values of mesh attributes.
/// If the attribute doesn't exist, returns none.
pub fn attr_to_vec(attr_fetch: Option<&mut VertexAttributeValues>) -> Option<Vec<Vec<&mut f32>>> {
    let attr = match attr_fetch {
        Some(attr) => attr,
        None => return None
    };

    match attr {
        VertexAttributeValues::Float32x3(vec) => {
            let mut return_vec: Vec<Vec<&mut f32>> = Vec::new();
            
            for i in vec.iter_mut() {
                let x = i.iter_mut().collect::<Vec<_>>();
                return_vec.push(x)
            }
            Some(return_vec)
        
        },
        VertexAttributeValues::Float32x2(vec) => {
            let mut return_vec: Vec<Vec<&mut f32>> = Vec::new();
            
            for i in vec.iter_mut() {
                let x = i.iter_mut().collect::<Vec<_>>();
                return_vec.push(x)
            }
            Some(return_vec)
        
        },
        err => panic!("attribute retrieval not implemented for {:#?}", err)
    }
}
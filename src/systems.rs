

use bevy_inspector_egui::bevy_inspector::guess_entity_name;
use bevy_state::prelude::*;
use bevy_input::prelude::*;
use bevy_ecs::world::CommandQueue;
use bevy_input::ButtonInput;
use bevy_log::warn;
use bevy_ecs::prelude::*;
use bevy_ecs::query::With;
use bevy_ecs::world::World;
use bevy_egui::EguiContext;
use bevy_utils::default;
use bevy_utils::hashbrown::HashMap;
use bevy_window::PresentMode;
use bevy_window::PrimaryWindow;
use bevy_window::Window;
use bevy_window::WindowResolution;
use egui::Ui;
use states::DebugMenuState;

use super::*;
//use crate::ui_for_entity_components;

pub fn nested_windows_test(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("sub windows test")
        .show(context.get_mut(), |ui| {
            egui::SidePanel::left("left")
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("left left left \n".repeat(500))
                }); 
            });
            egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Central Central Central \n".repeat(500))
                }); 
            });
            egui::SidePanel::right("right")
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("right right right \n".repeat(500))
                }); 
            })
        });
    }
}

pub fn manage_debug_menu_state(
    menu_controls: Res<UiExtrasKeybinds>,
    buttons: Res<ButtonInput<KeyCode>>,
    mut debug_menu_state_next: ResMut<NextState<DebugMenuState>>,
    debug_menu_state: Res<State<DebugMenuState>>

) {
    if buttons.just_pressed(menu_controls.toggle_debug_menu) {
        
      match debug_menu_state.get() {
        DebugMenuState::Open => debug_menu_state_next.set(DebugMenuState::Closed),
        DebugMenuState::Closed => debug_menu_state_next.set(DebugMenuState::Open),
          }
    }
}

pub fn debug_menu(world: &mut World) {
    type R = WindowStyleFrame;

    let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

    let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
        warn!("multiple \"primary\" windows found. This is not supported. Aborting");
        return;
    };
    let mut egui_context = egui_context_check.clone();

    //let components = world.components().iter().map(|n| n.name() );//.iter().filter_map(|n| n.type_id());
    let type_registry = world.resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();
    
    let Some(debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
        warn!("FilterResponse doesn't exist. Aborting");
        return;
    };
    let debug_filter_response = debug_filter_response.clone();

    
    let resources_filtered = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| {
            (
                //registration,
                registration.type_id(),
                registration.type_info().type_path_table().short_path(),
            )
        })
        .filter(|(_, name, ..)| {
            debug_filter_response.filter.len() <= 0 || name.to_lowercase().starts_with(&debug_filter_response.filter.to_lowercase())
        }) 
        .collect::<HashMap<TypeId, &str>>();
    let components_filtered = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectComponent>().is_some())
        
        .map(|registration| {
            (
                registration.type_id(),
                registration.type_info().type_path_table().short_path(),
            )
        })
        .filter(|(_ ,name, ..)| {
            debug_filter_response.filter.len() <= 0 || name.to_lowercase().starts_with(&debug_filter_response.filter.to_lowercase())
        }) 
        .collect::<HashMap<_, _>>();
    let components_filtered_and_attached = components_filtered
        .iter()
        .filter_map(|(id, name)| {
            let component_id  = match world.components().get_id(*id) {
                Some(id) => id,
                None => {
                    return None
                }
            };
            
            let mut query = QueryBuilder::<Entity>::new(world)
            .with_id(component_id)
            .build();

            let len = query.iter(world).len();

            if len > 0 {
                Some(
                    (
                        id,
                        (
                            name, 
                            query.iter(world).collect::<Vec<_>>()
                        )

                    )
                )
            } else {return None}
                
        })
        .collect::<HashMap<_, _>>();
    {
        egui::Window::new("debug_menu")
        
        .frame(window_style)
        //.auto_sized()
        //.scroll(true)
        .show(egui_context.get_mut(), |ui| {
            if ui.button("clear").clicked() {
                let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                    warn!("FilterResponse doesn't exist. Aborting");
                    return;
                };

                debug_filter_response.selected_type = None;
                debug_filter_response.filter = "".to_owned();
            }
            //if debug_filter_response.selected_type.is_none() {
            ui.horizontal(|ui| {
                ui.label("filter: ");
                let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                    warn!("FilterResponse doesn't exist. Aborting");
                    return;
                };
                ui.text_edit_singleline(&mut debug_filter_response.filter);
                //debug_filter_response.filter = debug_filter_response.filter.to_lowercase();

            });
            egui::SidePanel::left("Resources")
            .frame(window_style)
            .show_inside(ui, |ui| {
                let screen_size = ui.ctx().screen_rect().size();
                ui.set_max_size(screen_size);
                egui::ScrollArea::new(true)
                .show(ui, |ui| {
                    ui.heading("Resources");
                    for (id, name) in resources_filtered.iter() {
                        if ui.button(*name).clicked() {
                            let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                                warn!("FilterResponse doesn't exist. Aborting");
                                return;
                            };
                            debug_filter_response.selected_type = Some(TypeIdNameCache { type_id: *id, name: (*name).to_owned() })
                            
                        };
                    }
                });
            });
            egui::SidePanel::left("Components")
            .frame(window_style)

            .show_inside(ui, |ui| {
                let screen_size = ui.ctx().screen_rect().size();
                ui.set_max_size(screen_size);
                
                egui::ScrollArea::new(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
    

                            ui.heading("Components");
                            for (id, (name, _), ..) in components_filtered_and_attached.iter() {
                                if ui.button(**name).clicked() {
                                    let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                                        warn!("FilterResponse doesn't exist. Aborting");
                                        return;
                                    };
                                    debug_filter_response.selected_type = Some(TypeIdNameCache { type_id: **id, name: (**name).to_owned() })
                                    
                                };
                            }
                        });
                    });
                });

            });
            egui::SidePanel::left("results")
            .frame(window_style)
            .show_inside(ui, |ui| {


                let screen_size = ui.ctx().screen_rect().size();
                ui.set_max_size(screen_size);
                // let x_min = ui.ctx().used_rect().size().x;
                // let x_max = ui.ctx().screen_rect().size().x;
                // ui.set_width_range(0.0..=x_max);


                egui::ScrollArea::new(true)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        if let Some(resource) = debug_filter_response.selected_type {
                            ui.heading(&resource.name);
                            if components_filtered.contains_key(&resource.type_id) {
                                let mut queue = CommandQueue::default();
        
                                let Some((_, entities)) = components_filtered_and_attached.get(&resource.type_id) else {return;};
        
                                for entity in entities {
                                    let name = guess_entity_name(&world, *entity);
                                    ui.label(name);
                
                                    ui_for_component(
                                        &mut world.into(),
                                        Some(&mut queue),
                                        entity.clone(),
                                        ui,
                                        egui::Id::new(entity),
                                        &type_registry,
                                        &resource
                                    );
                                }
                
                                queue.apply(world);
                            }
                            if resources_filtered.contains_key(&resource.type_id) {
                                ui_for_resource(world, ui, &type_registry, &resource);
                            }
                        }
                    });
                })
            });

        });
    }
}


pub fn visualize_entities_with_component<T: Component>(display: Display) -> impl Fn(&mut World) {
    type R = WindowStyleFrame;
    
    let menu_name = std::any::type_name::<T>();

    move |world| {
        let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

        let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
            warn!("multiple \"primary\" windows found. This is not supported. Aborting");
            return;
        };
        let mut egui_context = egui_context_check.clone();


        let mut add_ui = {
            move |ui: &mut Ui | {
                ui.heading(menu_name);
                bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<T>>(
                    world, ui, true,
                );
            }
        };

        // ui
        match &display {
            Display::Side(side) => {
                let egui_side = match side {
                    Side::Left => egui::panel::Side::Left,
                    Side::Right => egui::panel::Side::Right,
                };
                egui::SidePanel::new(egui_side, menu_name)
                .frame(window_style)
                .show(egui_context.get_mut(), |ui| {
                    add_ui(ui)
                });
            }
            Display::Window => {
                egui::Window::new(menu_name)
                .frame(window_style)
                .show(egui_context.get_mut(), |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        add_ui(ui)
                    });
                });
            }
        };
    }

}

pub fn visualize_resource<T: Resource + Reflect>(display: Display) -> impl Fn(&mut World) {
    type R = WindowStyleFrame;
    let menu_name = std::any::type_name::<T>();

    move |world| {

        let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

        let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
            warn!("multiple \"primary\" windows found. This is not supported. Aborting");
            return;
        };

        let mut egui_context = egui_context_check.clone();

        let app_type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = app_type_registry.read();

        let resource = TypeIdNameCache::new_typed::<T>();
        let mut add_ui = {
            move |ui: &mut Ui | {

                let mut queue = CommandQueue::default();
                ui_for_resource(world, ui, &type_registry, &resource);

                queue.apply(world);
            }
        };

        match &display {
            Display::Side(side) => {
                let egui_side = match side {
                    Side::Left => egui::panel::Side::Left,
                    Side::Right => egui::panel::Side::Right,
                };
                egui::SidePanel::new(egui_side, menu_name)
                .frame(window_style)
                .show(egui_context.get_mut(), |ui| {
                    ui.heading(menu_name);
                    add_ui(ui)
                });
            }
            Display::Window => {
                egui::Window::new(menu_name)
                .frame(window_style)
                .show(egui_context.get_mut(), |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        add_ui(ui)
                    });
                });
            }
        };
    }
}

pub fn visualize_components_for<T: Component + Reflect>(display: Display) -> impl Fn(&mut World) {
    type R = WindowStyleFrame;
    let menu_name = std::any::type_name::<T>();

    move |world| {

        
        let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

        let component_entities = world.query_filtered::<Entity, With<T>>().iter(world).collect::<Vec<_>>();

        //last_componenent_entity = last_componenent_entity.clone();
        let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
            warn!("multiple \"primary\" windows found. This is not supported. Aborting");
            return;
        };
        let mut egui_context = egui_context_check.clone();

        let app_type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = app_type_registry.read();
        
        let add_ui = {
            move |ui: &mut Ui | {

                let mut queue = CommandQueue::default();
                let component = TypeIdNameCache::new_typed::<T>();

                for entity in component_entities {
                    let name = entity.to_string();

                    ui.label(name);

                    ui_for_component(
                        &mut world.into(),
                        Some(&mut queue),
                        entity.clone(),
                        ui,
                        egui::Id::new(entity),
                        &type_registry,
                        &component
                    );
                }

                queue.apply(world);
            }
        };

        match &display {
            Display::Side(side) => {
                let egui_side = match side {
                    Side::Left => egui::panel::Side::Left,
                    Side::Right => egui::panel::Side::Right,
                };
                egui::SidePanel::new(egui_side, menu_name)
                .frame(window_style)
                .show(egui_context.get_mut(), |ui| {
                    ui.heading(menu_name);
                    add_ui(ui)
                });
            }
            Display::Window => {
                egui::Window::new(menu_name)
                .frame(window_style)
                .show(egui_context.get_mut(), |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        add_ui(ui)
                    });
                });
            }
        };
    }
}

pub fn visualize_seperate_window_for_entities_with<T: Component>(
    world: &mut World,
    //mut commands: Commands,
) {
    let window_name = std::any::type_name::<T>();

    type R = WindowStyleFrame;
    let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, With<Visualize<T>>>()
        .get_single(world)
    {
        let mut egui_context = egui_context_check.clone();
        // // ui
        egui::CentralPanel::default()
            .frame(window_style)
            .show(egui_context.get_mut(), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(window_name);
                    bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<T>>(
                        world, ui, true,
                    );
                })
            });
    } else {
        // spawn a window if one doesn't exist for the component to visualize
        let window_length = (window_name.chars().count() as f32) * 10.0;
        world.spawn((
            Window {
                title: window_name.to_owned(),
                resolution: WindowResolution::new(window_length, 600.0),
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            Visualize::<T>::default(), //Name
        ));
    }
}
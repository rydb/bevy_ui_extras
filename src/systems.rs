

use std::collections::BTreeSet;
use std::ops::DerefMut;

use bevy_diagnostic::DiagnosticsStore;
use bevy_diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_diagnostic::SystemInformationDiagnosticsPlugin;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::bevy_inspector::guess_entity_name;
use bevy_inspector_egui::egui;
use bevy_inspector_egui::egui::Color32;
use bevy_inspector_egui::egui::FontFamily;
use bevy_inspector_egui::egui::FontId;
use bevy_inspector_egui::egui::RichText;
use bevy_inspector_egui::egui::Stroke;
use bevy_inspector_egui::egui::Ui;
use bevy_state::prelude::*;
use bevy_input::prelude::*;
use bevy_ecs::world::CommandQueue;
use bevy_input::ButtonInput;
use bevy_log::warn;
use bevy_ecs::prelude::*;
use bevy_ecs::query::With;
use bevy_ecs::world::World;
// use bevy_egui::EguiContext;
use bevy_utils::default;
use bevy_utils::hashbrown::HashMap;
use bevy_window::PresentMode;
use bevy_window::PrimaryWindow;
use bevy_window::Window;
use bevy_window::WindowResolution;
use colorgrad::Gradient;
// use egui::Color32;
// use egui::FontFamily;
// use egui::FontId;
// use egui::RichText;
// use egui::Stroke;
// use egui::Ui;
use states::DebugMenuState;
use strum::IntoEnumIterator;

use super::*;


pub(crate) fn set_entry_to_on<T: DerefMut<Target = bool> + Resource>(
    mut debug_mode_target: ResMut<T>,
) {
    **debug_mode_target = true;
}

pub(crate) fn set_entry_to_off<T: DerefMut<Target = bool> + Resource>(
    mut debug_mode_target: ResMut<T>,
) {
    **debug_mode_target = false;
}

/// displays misc info for app status
/// !!! CPU/RAM usage stats do not work when dynamic linking is enabled !!!
pub fn display_app_status(
    //mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    ui: &mut Ui,
    diagnostics: &DiagnosticsStore
) {
    let gray = Color32::GRAY;
    let font = FontId::new(20.0, FontFamily::default());

    let fps_grad = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0.0, 120.0])
    .build::<colorgrad::LinearGradient>().unwrap();

    let rev_grad = colorgrad::GradientBuilder::new()
    .html_colors(&["seagreen", "gold", "deeppink"])
    .domain(&[0.0, 100.0])
    .build::<colorgrad::LinearGradient>().unwrap();

    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
    .map(|diag| diag.value())
    .and_then(|n| n);

    let fps_color = fps
    .map(|n| {
        fps_grad.at(n as f32).to_array()
        .map(|n| n * 255.0)
        .map(|n| n as u8)
    })
    .map_or_else(|| gray, |n| {
        Color32::from_rgba_unmultiplied(n[0], n[1], n[2], n[3])
    })
    ;
    ui.horizontal(|ui| {
        ui.label(RichText::new("FPS:").font(font.clone()));
        ui.label(
            RichText::new(fps.map(|n| n.round().to_string()).unwrap_or("???".to_owned()))
            .color(fps_color)
            .font(font.clone())
        )
    });

    let cpu_usage = diagnostics.get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)
    .map(|diag| diag.value())
    .and_then(|n| n);
    let cpu_color = cpu_usage
    .map(|n| {
        rev_grad.at(n as f32).to_array()
        .map(|n| n * 255.0)
        .map(|n| n as u8)
    })
    .map_or_else(|| gray, |n| {
        Color32::from_rgba_unmultiplied(n[0], n[1], n[2], n[3])
    })
    ;

    ui.horizontal(|ui| {
        ui.label(RichText::new("CPU usage:").font(font.clone()));
        ui.label(
            RichText::new(cpu_usage.map(|n| n.round().to_string()).unwrap_or("???".to_owned()) + "%")
            .color(cpu_color)
            .font(font.clone())
        )
    });
    let ram_usage = diagnostics.get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)
    .map(|diag| diag.value())
    .and_then(|n| n);
    
    let ram_color = ram_usage
    .map(|n| {
        rev_grad.at(n as f32).to_array()
        .map(|n| n * 255.0)
        .map(|n| n as u8)
    })
    .map_or_else(|| gray, |n| {
        Color32::from_rgba_unmultiplied(n[0], n[1], n[2], n[3])
    });
    ui.horizontal(|ui| {
        ui.label(RichText::new("RAM usage:").font(font.clone()));
        ui.label(
            RichText::new(ram_usage.map(|n| n.round().to_string()).unwrap_or("???".to_owned()) + "%")
            .color(ram_color)
            .font(font.clone())
        )
    });
}

pub(crate) fn manage_debug_menu_state(
    menu_controls: Res<UiExtrasKeybinds>,
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_menu_state_next: ResMut<NextState<DebugMenuState>>,
    debug_menu_state: Res<State<DebugMenuState>>,
    mut focus_on_filter: ResMut<FocusOnDebugFilter>,
    mut filter: ResMut<FilterResponse>,
    mut debug_widget_view: ResMut<DebugWidgetView>,
) {

    if keys.just_pressed(menu_controls.toggle_debug_menu) {
      match debug_menu_state.get() {
        DebugMenuState::Open => debug_menu_state_next.set(DebugMenuState::Closed),
        DebugMenuState::Closed => debug_menu_state_next.set(DebugMenuState::Open),
          }
    }
    if keys.all_pressed(menu_controls.filter_quick_focus.clone()) {
        debug_menu_state_next.set(DebugMenuState::Open);
        focus_on_filter.0 = true;
    }
    if keys.all_pressed(menu_controls.clear.clone()) && debug_menu_state.get() == &DebugMenuState::Open {
        *filter = FilterResponse::default()
    }

    if keys.just_pressed(menu_controls.cycle_views) {
        match *debug_widget_view {
            DebugWidgetView::EntitiesView => *debug_widget_view = DebugWidgetView::ComponentsView ,
            DebugWidgetView::ComponentsView => *debug_widget_view = DebugWidgetView::EntitiesView,
        }
    }

    // if menu_controls.clear.iter().all(|key| keys.just_pressed(*key)) {
    //     debug_menu_state_next.set(DebugMenuState::Open);
    //     focus_on_filter.0 = true;
    // }
    // if keys.(menu_controls.clear) && debug_menu_state.get() == &DebugMenuState::Open {

    // }

}


pub fn debug_menu(world: &mut World) {
    type R = WindowStyleFrame;


    let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world)
    .inspect_err(|err| {
        warn!("No singleton primary window found. Aborting. Reason: {:#}", err);
    }) else {return;};
    let mut egui_context = egui_context_check.clone();

    let window_style = world.get_resource::<R>()
    .unwrap_or(&R::default()).0
    .unwrap_or(Frame::window(&egui_context.get_mut().style()))
    ;

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
            debug_filter_response.filter.len() <= 0 || name.to_lowercase().contains(&debug_filter_response.filter.to_lowercase())
        }) 
        .collect::<HashMap<_, _>>();
    
    let components_filtered = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectComponent>().is_some())    
        .map(|registration| {
            let type_path_table = registration.type_info().type_path_table();
            (
                registration.type_id(),
                (
                    type_path_table.short_path(),
                    type_path_table.path().split_once("::").unwrap_or(("ERROR GETTING SOURCE", "")).0
                )

            )
        })
        .filter(|(_ ,(name, ..), ..)| {
            debug_filter_response.filter.len() <= 0 || name.to_lowercase().contains(&debug_filter_response.filter.to_lowercase())
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
    let multi_select = world.get_resource::<ButtonInput<KeyCode>>()
    .map(|n| n.pressed(KeyCode::ShiftLeft))
    .unwrap_or(false);

    // let Some(selected_entities) = world.get_resource_mut::<SelectedEntities>() else {
    //     warn!("SelectedEntities doesn't exist. Aborting");
    //     return;
    // };
    // let selected_entities = selected_entities.clone();

    let Some(selected_widget) = world.get_resource_mut::<DebugWidgetView>() else {
        warn!("DebugWidgetView not found. Aborting");
        return;
    };

    let selected_widget = selected_widget.clone();
    {
        egui::Window::new("Debug Menu")        
        .frame(window_style)
        .show(egui_context.get_mut(), |ui| {
            if let Some(mut selected_widget) = world.get_resource_mut::<DebugWidgetView>() {
                ui.horizontal(|ui| {
                    for widget in DebugWidgetView::iter() {
                        let color = match *selected_widget == widget {
                            true => Color32::WHITE,
                            false => Color32::GRAY
                        };
                        
                        if ui.button(RichText::new(widget.to_string()).color(color)).clicked() {
                            *selected_widget = widget
                        }
                    }
                });

            }

            let mut show_app = false;
            if let Some(mut app_status) = world.get_resource_mut::<ShowAppStatus>() {
                let verb = match app_status.0 {
                    true => "close",
                    false => "open",
                };
                
                if ui.button(format!("{:#} app status", verb)).clicked() {
                    app_status.0 ^= true;
                }
                show_app = app_status.0;
            }
            if let Some(diagnostics) = world.get_resource::<DiagnosticsStore>(){
                if show_app {
                    ui.horizontal(|ui| {
                        display_app_status(ui, diagnostics);
                    });
                }
            }
            
            match selected_widget {
                DebugWidgetView::EntitiesView => {
                    egui::ScrollArea::both()
                    .show(ui, |ui| {
                        bevy_inspector::ui_for_world(world, ui);
                        //ui.allocate_space(ui.available_size());
                    });
                    return;

                },
                DebugWidgetView::ComponentsView => {
                
                },
            }
            if ui.button("clear").clicked() {
                let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                    warn!("FilterResponse doesn't exist. Aborting");
                    return;
                };

                debug_filter_response.selected_type.clear();
                debug_filter_response.filter = "".to_owned();
            }
            {
                let value = if let Some(debug_mode_toggle) = world.get_resource::<State<DebugModeFlagToggle>>() {
                    let mut value = match **debug_mode_toggle {
                        DebugModeFlagToggle::On => &mut true,
                        DebugModeFlagToggle::Off => &mut false,
                    };
                    ui.checkbox(&mut value, "Toggle Debug Mode");
                    Some(*value)
                } else {None};

                if let Some(value) = value {
                    if let Some(mut state) = world.get_resource_mut::<NextState<DebugModeFlagToggle>>() {
                        match value {
                            true => state.set(DebugModeFlagToggle::On),
                            false => state.set(DebugModeFlagToggle::Off),
                        }
                    }

                }
    
            }

            ui.horizontal(|ui| {
                ui.label("filter: ");
                let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                    warn!("FilterResponse doesn't exist. Aborting");
                    return;
                };
                let filter = ui.text_edit_singleline(&mut debug_filter_response.filter);
                if let Some(mut new_focus_request) = world.get_resource_mut::<FocusOnDebugFilter>() {
                    if new_focus_request.0 == true {
                        filter.request_focus();
                        new_focus_request.0 = false;
                    }
                }
            }); 
            // {
            //     let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
            //         warn!("FilterResponse doesn't exist. Aborting");
            //         return;
            //     }; 
            //     {
            //         ui.checkbox(&mut debug_filter_response.fuzzy_match_enabled, "Fuzzy Match");
    
            //     }
            // }

            // egui::SidePanel::left("Entities")
            // .frame(window_style)
            // .show_inside(ui, |ui| {
            //     let screen_size = ui.ctx().screen_rect().size();
            //     ui.set_max_size(screen_size);
            //     ui.heading("Entities");




            //     for entity in world.iter_entities().map(|e_ref| e_ref.id()){
                    
            //         let color = match selected_entities.0.contains(&entity) {
            //             true => Color32::WHITE,
            //             false => Color32::GRAY
            //         };

            //         let name = guess_entity_name(&world, entity);
                    


            //         if ui.button(RichText::new(name).color(color)).clicked() {
                        
            //             let Some(mut selected_entities) = world.get_resource_mut::<SelectedEntities>() else {
            //                 warn!("SelectedEntities doesn't exist. Aborting");
            //                 return;
            //             };
            //             selected_entities.0.insert(entity);
            //         }
            //     }
            //     //ui_for_world_entities_filtered::<Without<Parent>>(world, ui, true);

            // });

            egui::SidePanel::left("Resources")
            .frame(window_style)
            .show_inside(ui, |ui| {
                let screen_size = ui.ctx().screen_rect().size();
                ui.set_max_size(screen_size);
                egui::ScrollArea::new(true)
                .show(ui, |ui| {
                    ui.heading("Resources");
                    for (id, name) in resources_filtered.iter() {
                        let color = match debug_filter_response.selected_type.contains_key(id) {
                            true => Color32::WHITE,
                            false => Color32::GRAY
                        };
                        
                        if ui.button(RichText::new(*name).color(color)).clicked() {
                            let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                                warn!("FilterResponse doesn't exist. Aborting");
                                return;
                            };
                            let type_id_cache = TypeIdNameCache { type_id: *id, name: (**name).to_owned() };

                            if debug_filter_response.selected_type.get(id).is_some() {
                                if multi_select == false {
                                    debug_filter_response.selected_type.clear();
                                } else {
                                    debug_filter_response.selected_type.remove(id);
                                }
                            } else {
                                if multi_select == false {
                                    debug_filter_response.selected_type.clear();
                                }
                                debug_filter_response.selected_type.insert(*id, type_id_cache);
                            }
                            
                            
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
                            
                            let Some(mut match_mode) = world.get_resource_mut::<ComponentFilterMode>() else {
                                warn!("ComponentFilterMode doesn't exist. Aborting");
                                return;
                            };
                            egui::Frame::default()
                            .stroke(Stroke::new(2.0, Color32::BLACK))
                            .outer_margin(5.0)
                            .inner_margin(5.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    for variant in ComponentFilterMode::iter() {
                                        let color = match *match_mode == variant {
                                            true => Color32::WHITE,
                                            false => Color32::GRAY
                                        };
                    
                                        if ui.button(RichText::new(variant.to_string()).color(color)).clicked() {
                                            *match_mode = variant
                                        }
                                    }
                                });
                            });

                            
                            for (id, ((name, origin), _), ..) in components_filtered_and_attached.iter() {
                                let color = match debug_filter_response.selected_type.contains_key(*id) {
                                    true => Color32::WHITE,
                                    false => Color32::GRAY
                                };
                                let button = ui.button(RichText::new(*name).color(color)); 
                                
                                if button.clicked() {

                                    let Some(mut debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
                                        warn!("FilterResponse doesn't exist. Aborting");
                                        return;
                                    };
                                    let type_id_cache = TypeIdNameCache { type_id: **id, name: (**name).to_owned() };

                                    if debug_filter_response.selected_type.get(*id).is_some() {
                                        if multi_select == false {
                                            debug_filter_response.selected_type.clear();
                                        } else {
                                            debug_filter_response.selected_type.remove(*id);
                                        }
                                    } else {
                                        if multi_select == false {
                                            debug_filter_response.selected_type.clear();
                                        }
                                        debug_filter_response.selected_type.insert(**id, type_id_cache);
                                    }


                                };
                                if button.hovered() {
                                    ui.label(*origin);
                                }
                            }
                        });
                    });
                });

            });

            let selected_components = debug_filter_response.selected_type.iter()
            .filter(|(_, resource)| components_filtered.contains_key(&resource.type_id))
            .map(|(_, resource)| resource )
            .collect::<Vec<_>>();

            let selected_resources = debug_filter_response.selected_type.iter()
            .filter(|(_, resource)| resources_filtered.contains_key(&resource.type_id))
            .map(|(_, resource)| resource )
            .collect::<Vec<_>>();          
            
            
            egui::SidePanel::left("results".to_string())
            .frame(window_style)
            .show_inside(ui, |ui| {


                let screen_size = ui.ctx().screen_rect().size();
                ui.set_max_size(screen_size);

                egui::ScrollArea::new(true)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        let mut queue = CommandQueue::default();
                        let mut entities: BTreeSet<Entity> = BTreeSet::new();
                        
                        let Some(match_mode) = world.get_resource::<ComponentFilterMode>() else {
                            warn!("ComponentFilterMode doesn't exist. Aborting");
                            return;
                        };

                        match match_mode {
                            ComponentFilterMode::OR => {
                                let found = selected_components.iter()
                                .filter_map(|component| components_filtered_and_attached.get(&component.type_id))
                                .map(|(_, e)| e);
                                
                                for found_entities in found.into_iter() {
                                    for found_entity in found_entities.into_iter() {
                                        entities.insert(*found_entity);
                                    }
                                }
                            },
                            ComponentFilterMode::AND => {
                                let found = selected_components.iter()
                                .filter_map(|component| components_filtered_and_attached.get(&component.type_id))
                                .map(|(_, e)| e);

                                for found_entities in found.into_iter() {
                                    for found_entity in found_entities.into_iter() {
                                        let e = *found_entity;
                                        if selected_components.iter()
                                        .all(|comp| world.entity(e).contains_type_id(comp.type_id)) {
                                            entities.insert(e);
                                        }

                                    }
                                }
                            },
                        }
                        for entity in entities {
                            let name = guess_entity_name(&world, entity);
                            ui.label(name);
        
                            ui_for_components(
                                &mut world.into(),
                                Some(&mut queue),
                                entity,
                                ui,
                                egui::Id::new(entity),
                                &type_registry,
                                &selected_components
                            );
                        }
        
                        for resource in selected_resources.iter() {
                            ui.label(RichText::new(resource.name.clone()).color(Color32::WHITE));
                            ui_for_resource(
                                world, 
                                ui,
                                egui::Id::new(resource.type_id), 
                                &type_registry, 
                                &resource
                            );
                        }
                        queue.apply(world);

                    });
                })
            });
        });
    }
}

/// visualize all entities in a given format.
pub fn visualize_entities_with_component<T: Component>(display: Display) -> impl Fn(&mut World) {
    type R = WindowStyleFrame;
    
    let menu_name = std::any::type_name::<T>();

    move |world| {
        let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
            warn!("multiple \"primary\" windows found. This is not supported. Aborting");
            return;
        };
        let mut egui_context = egui_context_check.clone();

        let window_style = world.get_resource::<R>()
        .unwrap_or(&R::default()).0
        .unwrap_or(Frame::window(&egui_context.get_mut().style()));



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

/// visualize a resource with a given format.
pub fn visualize_resource<T: Resource + Reflect>(display: Display) -> impl Fn(&mut World) {
    type R = WindowStyleFrame;
    let menu_name = std::any::type_name::<T>();

    move |world| {
        let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
            warn!("multiple \"primary\" windows found. This is not supported. Aborting");
            return;
        };

        let mut egui_context = egui_context_check.clone();

        let window_style = world.get_resource::<R>()
        .unwrap_or(&R::default()).0
        .unwrap_or(Frame::window(&egui_context.get_mut().style()));

        let app_type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = app_type_registry.read();

        let resource = TypeIdNameCache::new_typed::<T>();
        let mut add_ui = {
            move |ui: &mut Ui | {

                let mut queue = CommandQueue::default();
                ui_for_resource(
                    world, 
                    ui, 
                    egui::Id::new(resource.type_id),
                    &type_registry, 
                    &resource
                );

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

/// visualize a given component with a given format.
pub fn visualize_components_for<T: Component + Reflect>(display: Display) -> impl Fn(&mut World) {
    type R = WindowStyleFrame;
    let menu_name = std::any::type_name::<T>();

    move |world| {
        let component_entities = world.query_filtered::<Entity, With<T>>().iter(world).collect::<Vec<_>>();

        let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
            warn!("multiple \"primary\" windows found. This is not supported. Aborting");
            return;
        };
        let mut egui_context = egui_context_check.clone();

        let window_style = world.get_resource::<R>()
        .unwrap_or(&R::default()).0
        .unwrap_or(Frame::window(&egui_context.get_mut().style()));

        let app_type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = app_type_registry.read();
        
        let add_ui = {
            move |ui: &mut Ui | {

                let mut queue = CommandQueue::default();
                let component = TypeIdNameCache::new_typed::<T>();

                for entity in component_entities {
                    let name = entity.to_string();

                    ui.label(name);

                    ui_for_components(
                        &mut world.into(),
                        Some(&mut queue),
                        entity,
                        ui,
                        egui::Id::new(entity),
                        &type_registry,
                        &vec![&component]
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

    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, With<Visualize<T>>>()
        .get_single(world)
    {
        let mut egui_context = egui_context_check.clone();

        let window_style = world.get_resource::<R>()
        .unwrap_or(&R::default()).0
        .unwrap_or(Frame::window(&egui_context.get_mut().style()));

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
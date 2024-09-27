

use bevy_diagnostic::DiagnosticsStore;
use bevy_diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_diagnostic::SystemInformationDiagnosticsPlugin;
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
use colorgrad::Gradient;
use egui::Color32;
use egui::FontFamily;
use egui::FontId;
use egui::RichText;
use egui::Ui;
use states::DebugMenuState;

use super::*;
//use crate::ui_for_entity_components;

/// converts to egui color

pub fn display_app_status(
    //mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    ui: &mut Ui,
    diagnostics: &DiagnosticsStore
) {
    let font = FontId::new(20.0, FontFamily::default());
    
    let grad = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .domain(&[0.0, 100.0])
    .build::<colorgrad::LinearGradient>().unwrap();

    let rev_grad = colorgrad::GradientBuilder::new()
    .html_colors(&["seagreen", "gold", "deeppink"])
    .domain(&[0.0, 100.0])
    .build::<colorgrad::LinearGradient>().unwrap();

    let gray = Color32::GRAY;
    let fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
    .map(|diag| diag.value())
    .and_then(|n| n);
    let fps_color = fps
    .map(|n| {
        grad.at(n as f32).to_array()
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
            RichText::new(cpu_usage.map(|n| n.to_string()).unwrap_or("???".to_owned()) + "%")
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
            RichText::new(ram_usage.map(|n| n.to_string()).unwrap_or("???".to_owned()) + "%")
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
    mut focus_on_filter: ResMut<FocusOnDebugFilter>
) {

    if keys.just_pressed(menu_controls.toggle_debug_menu) {
      match debug_menu_state.get() {
        DebugMenuState::Open => debug_menu_state_next.set(DebugMenuState::Closed),
        DebugMenuState::Closed => debug_menu_state_next.set(DebugMenuState::Open),
          }
    }
    if keys.any_just_pressed(menu_controls.filter_quick_focus.clone()) {
        debug_menu_state_next.set(DebugMenuState::Open);
        focus_on_filter.0 = true;
    }

}

pub fn debug_menu(world: &mut World) {
    type R = WindowStyleFrame;

    let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;



    let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world)
    .inspect_err(|err| {
        warn!("No singleton primary window found. Aborting. Reason: {:#}", err);
    }) else {return;};
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

            
            let mut show_app = false;
            if let Some(mut app_status) = world.get_resource_mut::<ShowAppStatus>() {
                if ui.button("show app status").clicked() {
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
                let filter = ui.text_edit_singleline(&mut debug_filter_response.filter);
                if let Some(mut new_focus_request) = world.get_resource_mut::<FocusOnDebugFilter>() {
                    if new_focus_request.0 == true {
                        filter.request_focus();
                        new_focus_request.0 = false;
                    }
                }
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
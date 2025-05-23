pub use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContext, egui::{self, Frame}};
use bevy_state::prelude::*;
use bevy_window::PrimaryWindow;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ops::DerefMut;

use bevy_diagnostic::DiagnosticsStore;
use bevy_diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_diagnostic::SystemInformationDiagnosticsPlugin;
use bevy_inspector_egui::bevy_inspector;
use bevy_inspector_egui::bevy_inspector::guess_entity_name;
use bevy_inspector_egui::egui::Color32;
use bevy_inspector_egui::egui::FontFamily;
use bevy_inspector_egui::egui::FontId;
use bevy_inspector_egui::egui::RichText;
// use bevy_inspector_egui::egui::Sense;
use bevy_ecs::prelude::*;
use bevy_ecs::query::With;
use bevy_ecs::world::CommandQueue;
use bevy_ecs::world::World;
use bevy_input::ButtonInput;
use bevy_input::prelude::*;
use bevy_inspector_egui::egui::Slider;
use bevy_inspector_egui::egui::Stroke;
use bevy_inspector_egui::egui::Ui;
use bevy_log::warn;
use bevy_state::prelude::*;
// use bevy_egui::EguiContext;
use bevy_utils::default;
use bevy_window::PresentMode;
use bevy_window::Window;
use bevy_window::WindowResolution;
use colorgrad::Gradient;
use strum::IntoEnumIterator;

use crate::widgets::{app_status::resources::ShowAppStatus, components_view::ui_for_components};
use crate::ui_for_resource;
use crate::widgets::app_status::display_app_status;
use crate::Opacity;
use crate::TypeIdNameCache;
use crate::UiAlignment;
use crate::UiStyle;
use crate::OPACITY_RANGE;

use super::{*, resources::*,};

pub(crate) fn manage_debug_menu_state(
    menu_controls: Res<DebugMenuKeybinds>,
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
            DebugMenuState::Explain => debug_menu_state_next.set(DebugMenuState::Open),
        }
    }
    if keys.all_pressed(menu_controls.filter_quick_focus.clone()) {
        debug_menu_state_next.set(DebugMenuState::Open);
        focus_on_filter.0 = true;
    }
    if keys.all_pressed(menu_controls.clear.clone())
        && debug_menu_state.get() == &DebugMenuState::Open
    {
        *filter = FilterResponse::default()
    }

    if keys.just_pressed(menu_controls.cycle_views) {
        match *debug_widget_view {
            DebugWidgetView::EntitiesView => *debug_widget_view = DebugWidgetView::ComponentsView,
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

pub fn display_debug_menu_explanation(
    mut windows: Query<&mut EguiContext, With<PrimaryWindow>>,
    controls: Res<DebugMenuKeybinds>,
    alignment: Res<UiAlignment>,
    frame: Res<UiStyle>,
) {
    let Ok(mut context) = windows.single_mut() else {
        return;
    };

    let mut window = egui::Window::new("Controls");

    let frame = match frame.0 {
        Some(frame) => frame,
        None => Frame::window(&context.get_mut().style()),
    };
    match alignment.0 {
        Some(alignment) => {
            window = window.anchor(alignment, [0.0, 0.0]);
        }
        None => {}
    }
    window
        .frame(frame)
        //.anchor(alignment.0, [0.0, 0.0])
        .show(context.get_mut(), |ui| {
            ui.label(format!("{:#?}", controls));
        });
}

pub fn debug_menu(world: &mut World) {
    type R = UiStyle;

    let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .inspect_err(|err| {
            warn!(
                "No singleton primary window found. Aborting. Reason: {:#}",
                err
            );
        })
    else {
        return;
    };
    let mut egui_context = egui_context_check.clone();

    let mut window_style = world
        .get_resource::<R>()
        .unwrap_or(&R::default())
        .0
        .unwrap_or(Frame::window(&egui_context.get_mut().style()));

    {
        if let Some(opacity) = world.get_resource::<Opacity>() {
            window_style.fill[3] = opacity.0
        }
    }
    let alignment = {
        let Some(alignment) = world.get_resource::<UiAlignment>() else {
            return;
        };
        alignment.0
    };

    //let components = world.components().iter().map(|n| n.name() );//.iter().filter_map(|n| n.type_id());
    let type_registry = world.resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();

    let Some(debug_filter_response) = world.get_resource_mut::<FilterResponse>() else {
        warn!("FilterResponse doesn't exist. Aborting");
        return;
    };
    let debug_filter_response = debug_filter_response.clone();

    let resources_filtered = {
        let resources = type_registry
            .iter()
            .filter(|registration| registration.data::<ReflectResource>().is_some())
            .map(|registration| {
                (
                    //registration,
                    registration.type_id(),
                    registration.type_info().type_path_table().short_path(),
                )
            });
        // for filter in debug_filter_response.filters.into_iter() {
        //     match filter {
        //         FilterKind::Crate(name) => {
        //             let resources = resources.filter(predicate)
        //         },
        //         FilterKind::Name(name) => todo!(),
        //     }
        // }
        let resources = resources
            .filter(|(_, name, ..)| {
                debug_filter_response.filter_prompt.len() <= 0
                    || name
                        .to_lowercase()
                        .contains(&debug_filter_response.filter_prompt.to_lowercase())
            })
            .collect::<HashMap<_, _>>();
        resources
    };
    let components_filtered = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectComponent>().is_some())
        .map(|registration| {
            let type_path_table = registration.type_info().type_path_table();
            (
                registration.type_id(),
                (
                    type_path_table.short_path(),
                    type_path_table
                        .path()
                        .split_once("::")
                        .unwrap_or(("ERROR GETTING SOURCE", ""))
                        .0,
                ),
            )
        })
        .filter(|(_, (name, ..), ..)| {
            debug_filter_response.filter_prompt.len() <= 0
                || name
                    .to_lowercase()
                    .contains(&debug_filter_response.filter_prompt.to_lowercase())
        })
        .collect::<HashMap<_, _>>();

    let components_filtered_and_attached = components_filtered
        .iter()
        .filter_map(|(id, name)| {
            let component_id = match world.components().get_id(*id) {
                Some(id) => id,
                None => return None,
            };

            let mut query = QueryBuilder::<Entity>::new(world)
                .with_id(component_id)
                .build();

            let len = query.iter(world).len();

            if len > 0 {
                Some((id, (name, query.iter(world).collect::<Vec<_>>())))
            } else {
                return None;
            }
        })
        .collect::<HashMap<_, _>>();

    let multi_select = world
        .get_resource::<ButtonInput<KeyCode>>()
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
        let mut window = egui::Window::new("Debug Menu");

        if let Some(alignment) = alignment {
            window = window.anchor(alignment, [0.0, 0.0])
        }
        window
            .frame(window_style)
            //.anchor(alignment, [0.0, 0.0])
            .show(egui_context.get_mut(), |ui| {
                {}
                if let Some(mut opacity) = world.get_resource_mut::<Opacity>() {
                    ui.horizontal(|ui| {
                        ui.label("Opacity");
                        ui.add(Slider::new(&mut opacity.0, OPACITY_RANGE));
                        if ui.button("Glass").clicked() {
                            opacity.0 = 128;
                        }
                        if ui.button("Opaque").clicked() {
                            opacity.0 = 255;
                        }
                    });

                    //warn!("could not get opacity");
                }

                if let Some(mut selected_widget) = world.get_resource_mut::<DebugWidgetView>() {
                    ui.horizontal(|ui| {
                        for widget in DebugWidgetView::iter() {
                            let color = match *selected_widget == widget {
                                true => Color32::WHITE,
                                false => Color32::GRAY,
                            };

                            if ui
                                .button(RichText::new(widget.to_string()).color(color))
                                .clicked()
                            {
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
                if let Some(diagnostics) = world.get_resource::<DiagnosticsStore>() {
                    if show_app {
                        ui.horizontal(|ui| {
                            display_app_status(ui, diagnostics);
                        });
                    }
                }

                match selected_widget {
                    DebugWidgetView::EntitiesView => {
                        egui::ScrollArea::both().show(ui, |ui| {
                            bevy_inspector::ui_for_world(world, ui);
                            //ui.allocate_space(ui.available_size());
                        });
                        return;
                    }
                    DebugWidgetView::ComponentsView => {}
                }
                if ui.button("clear").clicked() {
                    let Some(mut debug_filter_response) =
                        world.get_resource_mut::<FilterResponse>()
                    else {
                        warn!("FilterResponse doesn't exist. Aborting");
                        return;
                    };

                    debug_filter_response.selected_type.clear();
                    debug_filter_response.filter_prompt = "".to_owned();
                }
                {
                    let value = if let Some(debug_mode_toggle) =
                        world.get_resource::<State<DebugModeFlagToggle>>()
                    {
                        let mut value = match **debug_mode_toggle {
                            DebugModeFlagToggle::On => &mut true,
                            DebugModeFlagToggle::Off => &mut false,
                        };
                        ui.checkbox(&mut value, "Toggle Debug Mode");
                        Some(*value)
                    } else {
                        None
                    };

                    if let Some(value) = value {
                        if let Some(mut state) =
                            world.get_resource_mut::<NextState<DebugModeFlagToggle>>()
                        {
                            match value {
                                true => state.set(DebugModeFlagToggle::On),
                                false => state.set(DebugModeFlagToggle::Off),
                            }
                        }
                    }
                }

                ui.horizontal(|ui| {
                    ui.label("filter: ");

                    // let pressed_enter = {
                    //     let Some(keys) = world.get_resource::<ButtonInput<KeyCode>>() else {
                    //         return;
                    //     };

                    //     if keys.just_pressed(KeyCode::Enter) == true {
                    //         true
                    //     } else {
                    //         false
                    //     }
                    // };

                    let Some(mut debug_filter_response) =
                        world.get_resource_mut::<FilterResponse>()
                    else {
                        warn!("FilterResponse doesn't exist. Aborting");
                        return;
                    };
                    let filter = ui.text_edit_singleline(&mut debug_filter_response.filter_prompt);
                    {
                        //TODO: implement this when filters expanded more.
                        // if pressed_enter {
                        //     //println!("adding new filter");
                        //     let mut new_filters = vec![FilterKind::Name(
                        //         debug_filter_response.filter_prompt.clone(),
                        //     )];
                        //     new_filters.extend(debug_filter_response.filters.clone());
                        //     debug_filter_response.filters = new_filters;
                        // }
                    }
                    if let Some(mut new_focus_request) =
                        world.get_resource_mut::<FocusOnDebugFilter>()
                    {
                        if new_focus_request.0 == true {
                            filter.request_focus();
                            new_focus_request.0 = false;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("filters: ");
                    let Some(filters) = world.get_resource::<FilterResponse>() else {
                        return;
                    };
                    for filter in filters.filters.clone().into_iter() {
                        match filter {
                            FilterKind::Crate(name) => {
                                ui.label(format!("In crate: {:#}", name));
                            }
                            FilterKind::Name(name) => {
                                ui.label(format!("has component: {:#}", name));
                            }
                        }
                    }
                });

                egui::SidePanel::left("Resources")
                    .frame(window_style)
                    .show_inside(ui, |ui| {
                        let screen_size = ui.ctx().screen_rect().size();
                        ui.set_max_size(screen_size);
                        egui::ScrollArea::new(true).show(ui, |ui| {
                            ui.heading("Resources");

                            let mut alphabetized_resources =
                                resources_filtered.iter().collect::<Vec<_>>();

                            alphabetized_resources.sort_by(|(_, name), (_, name2)| name.cmp(name2));
                            for (id, name) in alphabetized_resources.iter() {
                                let color =
                                    match debug_filter_response.selected_type.contains_key(id) {
                                        true => Color32::WHITE,
                                        false => Color32::GRAY,
                                    };

                                if ui.button(RichText::new(**name).color(color)).clicked() {
                                    let Some(mut debug_filter_response) =
                                        world.get_resource_mut::<FilterResponse>()
                                    else {
                                        warn!("FilterResponse doesn't exist. Aborting");
                                        return;
                                    };
                                    let type_id_cache = TypeIdNameCache {
                                        type_id: **id,
                                        name: (**name).to_owned(),
                                    };

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
                                        debug_filter_response
                                            .selected_type
                                            .insert(**id, type_id_cache);
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

                        egui::ScrollArea::new(true).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.heading("Components");

                                    // let Some(mut match_mode) =
                                    //     world.get_resource_mut::<ComponentFilterMode>()
                                    // else {
                                    //     warn!("ComponentFilterMode doesn't exist. Aborting");
                                    //     return;
                                    // };
                                    egui::Frame::default()
                                        .stroke(Stroke::new(2.0, Color32::BLACK))
                                        .outer_margin(5.0)
                                        .inner_margin(5.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                // for variant in ComponentFilterMode::iter() {
                                                //     let color = match *match_mode == variant {
                                                //         true => Color32::WHITE,
                                                //         false => Color32::GRAY,
                                                //     };

                                                //     if ui
                                                //         .button(
                                                //             RichText::new(variant.to_string())
                                                //                 .color(color),
                                                //         )
                                                //         .clicked()
                                                //     {
                                                //         *match_mode = variant
                                                //     }
                                                // }
                                            });
                                        });
                                    let mut alphabetized_components =
                                        components_filtered.iter().collect::<Vec<_>>();

                                    alphabetized_components
                                        .sort_by(|(_, (name, _)), (_, (name2, _))| name.cmp(name2));
                                    for (id, (name, origin)) in alphabetized_components.iter() {
                                        let color = match debug_filter_response
                                            .selected_type
                                            .contains_key(*id)
                                        {
                                            true => Color32::WHITE,
                                            false => Color32::GRAY,
                                        };
                                        let button = ui.button(RichText::new(*name).color(color));

                                        if button.clicked() {
                                            let Some(mut debug_filter_response) =
                                                world.get_resource_mut::<FilterResponse>()
                                            else {
                                                warn!("FilterResponse doesn't exist. Aborting");
                                                return;
                                            };
                                            let type_id_cache = TypeIdNameCache {
                                                type_id: **id,
                                                name: (**name).to_owned(),
                                            };

                                            if debug_filter_response
                                                .selected_type
                                                .get(*id)
                                                .is_some()
                                            {
                                                if multi_select == false {
                                                    debug_filter_response.selected_type.clear();
                                                } else {
                                                    debug_filter_response.selected_type.remove(*id);
                                                }
                                            } else {
                                                if multi_select == false {
                                                    debug_filter_response.selected_type.clear();
                                                }
                                                debug_filter_response
                                                    .selected_type
                                                    .insert(**id, type_id_cache);
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

                let selected_components = debug_filter_response
                    .selected_type
                    .iter()
                    .filter(|(_, resource)| components_filtered.contains_key(&resource.type_id))
                    .map(|(_, resource)| resource)
                    .collect::<Vec<_>>();

                let selected_resources = debug_filter_response
                    .selected_type
                    .iter()
                    .filter(|(_, resource)| resources_filtered.contains_key(&resource.type_id))
                    .map(|(_, resource)| resource)
                    .collect::<Vec<_>>();

                egui::SidePanel::left("results".to_string())
                    .frame(window_style)
                    .show_inside(ui, |ui| {
                        let screen_size = ui.ctx().screen_rect().size();
                        ui.set_max_size(screen_size);

                        egui::ScrollArea::new(true).show(ui, |ui| {
                            ui.vertical(|ui| {
                                let mut queue = CommandQueue::default();
                                let mut entities: BTreeSet<Entity> = BTreeSet::new();

                                // let Some(match_mode) = world.get_resource::<ComponentFilterMode>()
                                // else {
                                //     warn!("ComponentFilterMode doesn't exist. Aborting");
                                //     return;
                                // };

                                // match match_mode {
                                //     ComponentFilterMode::OR => {
                                //         let found = selected_components
                                //             .iter()
                                //             .filter_map(|component| {
                                //                 components_filtered_and_attached
                                //                     .get(&component.type_id)
                                //             })
                                //             .map(|(_, e)| e);

                                //         for found_entities in found.into_iter() {
                                //             for found_entity in found_entities.into_iter() {
                                //                 entities.insert(*found_entity);
                                //             }
                                //         }
                                //     }
                                //     ComponentFilterMode::AND => {
                                //         let found = selected_components
                                //             .iter()
                                //             .filter_map(|component| {
                                //                 components_filtered_and_attached
                                //                     .get(&component.type_id)
                                //             })
                                //             .map(|(_, e)| e);

                                //         for found_entities in found.into_iter() {
                                //             for found_entity in found_entities.into_iter() {
                                //                 let e = *found_entity;
                                //                 if selected_components.iter().all(|comp| {
                                //                     world.entity(e).contains_type_id(comp.type_id)
                                //                 }) {
                                //                     entities.insert(e);
                                //                 }
                                //             }
                                //         }
                                //     }
                                // }
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
                                        &selected_components,
                                    );
                                }

                                for resource in selected_resources.iter() {
                                    ui.label(
                                        RichText::new(resource.name.clone()).color(Color32::WHITE),
                                    );
                                    ui_for_resource(
                                        world,
                                        ui,
                                        egui::Id::new(resource.type_id),
                                        &type_registry,
                                        &resource,
                                    );
                                }
                                queue.apply(world);
                            });
                        })
                    });
            });
    }
}
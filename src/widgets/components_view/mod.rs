pub use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContext, egui::{self, Frame}, reflect_inspector::{Context, InspectorUi}, restricted_world_view::RestrictedWorldView};
use bevy_reflect::{Reflect, TypeRegistry};
use bevy_state::prelude::*;
use bevy_window::PrimaryWindow;
use std::{any::TypeId, collections::BTreeSet};
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
use bevy_ecs::{component::ComponentId, prelude::*};
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

use crate::{Display, Side, TypeIdNameCache, UiStyle};

pub mod plugins;
mod resources;
mod systems;

/// displays ui for entity and its given components
pub fn ui_for_components(
    world: &mut RestrictedWorldView<'_>,
    mut queue: Option<&mut CommandQueue>,
    entity: Entity,
    ui: &mut egui::Ui,
    id: egui::Id,
    type_registry: &TypeRegistry,
    components: &Vec<&TypeIdNameCache>,
) {
    for component in components.iter() {
        let Some((name, component_id, component_type_id, size)) =
            component_info_for(world, component)
        else {
            return;
        };

        let id = id.with(component_id);

        let header = egui::CollapsingHeader::new(&name)
            .id_salt(id)
            .default_open(false);

        let Some(component_type_id) = component_type_id else {
            return;
        };

        // create a context with access to the world except for the currently viewed component
        let (mut component_view, world) = world.split_off_component((entity, component_type_id));
        let mut cx = Context {
            world: Some(world),
            #[allow(clippy::needless_option_as_deref)]
            queue: queue.as_deref_mut(),
        };

        // let (value, _, set_changed) = match component_view.get_entity_component_reflect(
        //     entity,
        //     component_type_id,
        //     type_registry,
        // ) {
        //     Ok(value) => value,
        //     Err(_) => {
        //         //header.show(ui, |ui| errors::show_error(e, ui, &name));
        //         //ui.label(format!("{:#?}", e));
        //         continue;
        //     }
        // };
        let Ok(mut value) = component_view
            .get_entity_component_reflect(entity, component_type_id, type_registry)
            .inspect_err(|err| {
                // skip over errors that are not relevant(Not having given component)
                let mabye_err = match err {
                    bevy_inspector_egui::restricted_world_view::Error::NoAccessToResource(_) => {
                        Some(err)
                    }
                    bevy_inspector_egui::restricted_world_view::Error::NoAccessToComponent(_) => {
                        Some(err)
                    }
                    bevy_inspector_egui::restricted_world_view::Error::ResourceDoesNotExist(_) => {
                        None
                    }
                    bevy_inspector_egui::restricted_world_view::Error::ComponentDoesNotExist(_) => {
                        None
                    }
                    bevy_inspector_egui::restricted_world_view::Error::NoComponentId(_) => {
                        Some(err)
                    }
                    bevy_inspector_egui::restricted_world_view::Error::NoTypeRegistration(_) => {
                        Some(err)
                    }
                    bevy_inspector_egui::restricted_world_view::Error::NoTypeData(_, _) => {
                        Some(err)
                    }
                };

                if let Some(msg) = mabye_err {
                    ui.label(format!("{:#?}", msg));
                }
            })
        else {
            continue;
        };

        if size == 0 {
            header.show(ui, |_| {});
            continue;
        }

        // if is_changed {
        //     #[cfg(feature = "highlight_changes")]
        //     set_highlight_style(ui);
        // }

        header.show(ui, |ui| {
            ui.reset_style();

            let inspector_changed = InspectorUi::for_bevy(type_registry, &mut cx)
                .ui_for_reflect_with_options(
                    value.bypass_change_detection().as_partial_reflect_mut(),
                    ui,
                    id.with(component_id),
                    &(),
                );

            if inspector_changed {
                value.set_changed();
            }
        });
        ui.reset_style();
    }
}

/// fetches the info for the componenet of type T for the given entity, if it exists.
pub fn component_info_for(
    world: &mut RestrictedWorldView<'_>,
    component: &TypeIdNameCache,
) -> Option<(String, ComponentId, Option<TypeId>, usize)> {
    let component_id = match world.world().components().get_id(component.type_id) {
        Some(id) => id,
        None => {
            warn!("Could not get component id for {:#}", component.name);
            return None;
        }
    };
    let info = world.world().components().get_info(component_id)?;

    let name = pretty_type_name::pretty_type_name_str(info.name());

    return Some((name, component_id, info.type_id(), info.layout().size()));
}

/// visualize a given component with a given format.
pub fn visualize_components_for<T: Component + Reflect>(display: Display) -> impl Fn(&mut World) {
    type R = UiStyle;
    let menu_name = std::any::type_name::<T>();

    move |world| {
        let component_entities = world
            .query_filtered::<Entity, With<T>>()
            .iter(world)
            .collect::<Vec<_>>();

        let Ok(egui_context_check) = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .single(world)
        else {
            warn!("multiple \"primary\" windows found. This is not supported. Aborting");
            return;
        };
        let mut egui_context = egui_context_check.clone();

        let window_style = world
            .get_resource::<R>()
            .unwrap_or(&R::default())
            .0
            .unwrap_or(Frame::window(&egui_context.get_mut().style()));

        let app_type_registry = world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = app_type_registry.read();

        let add_ui = {
            move |ui: &mut Ui| {
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
                        &vec![&component],
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
                egui::Window::new(menu_name).frame(window_style).show(
                    egui_context.get_mut(),
                    |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| add_ui(ui));
                    },
                );
            }
        };
    }
}
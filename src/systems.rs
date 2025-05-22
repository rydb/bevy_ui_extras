use std::collections::BTreeSet;
use std::collections::HashMap;
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
use bevy_window::PrimaryWindow;
use bevy_window::Window;
use bevy_window::WindowResolution;
use colorgrad::Gradient;
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

/// visualize all entities in a given format.
pub fn visualize_entities_with_component<T: Component>(display: Display) -> impl Fn(&mut World) {
    type R = UiStyle;

    let menu_name = std::any::type_name::<T>();

    move |world| {
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

        let mut add_ui = {
            move |ui: &mut Ui| {
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
                    .show(egui_context.get_mut(), |ui| add_ui(ui));
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

/// visualize a resource with a given format.
pub fn visualize_resource<T: Resource + Reflect>(display: Display) -> impl Fn(&mut World) {
    type R = UiStyle;
    let menu_name = std::any::type_name::<T>();

    move |world| {
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

        let resource = TypeIdNameCache::new_typed::<T>();
        let mut add_ui = {
            move |ui: &mut Ui| {
                let mut queue = CommandQueue::default();
                ui_for_resource(
                    world,
                    ui,
                    egui::Id::new(resource.type_id),
                    &type_registry,
                    &resource,
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

pub fn visualize_seperate_window_for_entities_with<T: Component>(
    world: &mut World,
    //mut commands: Commands,
) {
    let window_name = std::any::type_name::<T>();

    type R = UiStyle;

    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, With<Visualize<T>>>()
        .get_single(world)
    {
        let mut egui_context = egui_context_check.clone();

        let window_style = world
            .get_resource::<R>()
            .unwrap_or(&R::default())
            .0
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

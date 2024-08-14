
use bevy_ecs::world::CommandQueue;
use bevy_log::warn;
use bevy_ecs::prelude::*;
use bevy_ecs::query::With;
use bevy_ecs::world::World;
use bevy_egui::EguiContext;
use bevy_utils::default;
use bevy_window::PresentMode;
use bevy_window::PrimaryWindow;
use bevy_window::Window;
use bevy_window::WindowResolution;
use egui::Ui;

use super::*;
//use crate::ui_for_entity_components;

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
                for component in component_entities {
                    let name = component.to_string();

                    ui.label(name);

                    ui_for_component::<T>(
                        &mut world.into(),
                        Some(&mut queue),
                        component.clone(),
                        ui,
                        egui::Id::new(component),
                        &type_registry,
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
use std::ops::DerefMut;

use bevy_ecs::world::CommandQueue;
use bevy_log::warn;
use bevy_ecs::prelude::*;
use bevy_ecs::query::With;
use bevy_ecs::system::SystemState;
use bevy_ecs::world::World;
use bevy_egui::EguiContext;
use bevy_inspector_egui::reflect_inspector;
use bevy_reflect::Reflect;
use bevy_utils::default;
use bevy_window::PresentMode;
use bevy_window::PrimaryWindow;
use bevy_window::Window;
use bevy_window::WindowResolution;

use crate::components::Visualize;
use crate::resources::WindowStyleFrame;
use crate::ui_for_component;
//use crate::ui_for_entity_components;

pub fn visualize_left_sidepanel_for<T: Component>(world: &mut World) {
    type R = WindowStyleFrame;
    let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    {
        let menu_name = std::any::type_name::<T>();

        let mut egui_context = egui_context_check.clone();

        // // ui

        egui::SidePanel::new(egui::panel::Side::Left, menu_name)
            .frame(window_style)
            .show(egui_context.get_mut(), |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(menu_name);
                    bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<T>>(
                        world, ui, true,
                    );
                })
            });
    }
}

pub fn visualize_right_sidepanel_for<T: Component>(world: &mut World) {
    type R = WindowStyleFrame;
    let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

    let mut component_entities = world.query_filtered::<Entity, With<T>>().iter(world).collect::<Vec<_>>() else {return;}; 
    //last_componenent_entity = last_componenent_entity.clone();
    let Ok(egui_context_check) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else {
        warn!("multiple \"primary\" windows found. This is not supported. Aborting");
        return;
    };

    
    let app_type_registry = world.resource::<AppTypeRegistry>().0.clone();
    let type_registry = app_type_registry.read();


    let mut egui_context = egui_context_check.clone();



    let menu_name = std::any::type_name::<T>();


    // ui

    egui::SidePanel::new(egui::panel::Side::Right, menu_name)
        .frame(window_style)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut queue = CommandQueue::default();

                for component in component_entities.iter() {
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

            })
        });
    
}

// pub fn visualize_right_sidepanel_for_componenent<T: Component + Reflect>(mut world: &mut World) {
//     type R = WindowStyleFrame;
    
//     let mut system_state: SystemState<(
//         Res<R>,
//         Res<AppTypeRegistry>,
//         Query<&mut EguiContext, With<PrimaryWindow>>,
//         Query<&mut T>,
//     )> = SystemState::new(&mut world);

//     let (window_style, app_type_registry, contexts, mut components) = system_state.get_mut(&mut world);

//     let Ok(egui_context) = contexts.get_single() else {
//         warn!("multiple \"primary\" windows found. This is not supported. Aborting");
//         return;
//     };

//     let Some(mut last_componenent) = components.iter_mut().last() else{ return;};
//     let type_registry = app_type_registry.read();

//     let menu_name = std::any::type_name::<T>();

//     let mut egui_context = egui_context.clone();


//     egui::SidePanel::new(egui::panel::Side::Right, menu_name)
//         .frame(window_style.frame)
//         .show(egui_context.get_mut(), |ui| {
//             egui::ScrollArea::vertical().show(ui, |ui| {
//                 ui.heading(menu_name);
//                 bevy_inspector_egui::bevy_inspector::ui_for_entity::<With<T>>(
//                     world, ui, true,
//                 );
//                 reflect_inspector::ui_for_value(last_componenent.as_reflect_mut(), ui, &type_registry)
//                 // !!! Just ignore getting the handle underyling value for brevity and use reflector_ui.. !!!
//                 // for mut component in components.iter_mut() {
//                 //     reflect_inspector::ui_for_value(
//                 //     &mut *component, ui, &type_registry,
//                 //     );
//                 // }
        


//             })
//         });
// }

pub fn visualize_seperate_window<T: Component>(
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

pub fn visualize_window_for<T: Component>(
    world: &mut World,
    //mut commands: Commands,
) {
    let window_name = std::any::type_name::<T>();

    type R = WindowStyleFrame;
    let window_style = world.get_resource::<R>().unwrap_or(&R::default()).frame;

    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    {
        let mut egui_context = egui_context_check.clone();
        // // ui
        //let frame = egui::Frame::window(ui.style()).fill(egui::Color32::from_rgba_premultiplied(30,30,30, 128))
        egui::Window::new(window_name)
            .frame(window_style)
            .show(egui_context.get_mut(), |ui| {
                //let frame = egui::Frame::window(ui.style()).fill(egui::Color32::from_rgba_premultiplied(30,30,30, 128));
                //ui.set_style(style)
                egui::ScrollArea::vertical().show(ui, |ui| {
                    //ui.heading(window_name);
                    bevy_inspector_egui::bevy_inspector::ui_for_world_entities_filtered::<With<T>>(
                        world, ui, true,
                    );
                })
            });
    }
}

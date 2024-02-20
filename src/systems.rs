use bevy_ecs::component::Component;
use bevy_ecs::query::With;
use bevy_ecs::world::World;
use bevy_egui::EguiContext;
use bevy_utils::default;
use bevy_window::PresentMode;
use bevy_window::PrimaryWindow;
use bevy_window::Window;
use bevy_window::WindowResolution;

use crate::components::Visualize;
use crate::resources::WindowStyleFrame;

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

    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    {
        let menu_name = std::any::type_name::<T>();

        let mut egui_context = egui_context_check.clone();

        // ui

        egui::SidePanel::new(egui::panel::Side::Right, menu_name)
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

use std::marker::PhantomData;
use std::ops::DerefMut;

use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_state::prelude::*;
use bevy_app::{Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{manage_debug_menu_state, set_entry_to_off, set_entry_to_on, ComponentFilterMode, DebugMenuToggle, DebugModeToggle, DebugWidgetView, FocusOnDebugFilter, ShowAppStatus, UiStyle, WindowStyleFrame};
use crate::{debug_menu, states::DebugMenuState, FilterResponse, UiExtrasKeybinds};



/// plugin for general debug menu. See [`UiExtrasKeybinds`] for keybinds. 
pub struct UiExtrasDebug {
    pub ui_style: UiStyle,
    pub keybinds_override: Option<UiExtrasKeybinds>,
}

impl Default for UiExtrasDebug {
    fn default() -> Self {
        Self {
            ui_style: UiStyle::BlackGlass,
            keybinds_override: None
        }
    }
}

impl Plugin for UiExtrasDebug {
    fn build(&self, app: &mut bevy_app::App) {
        let window_style = match self.ui_style {
            UiStyle::BlackGlass => WindowStyleFrame::default(),
            UiStyle::Default => WindowStyleFrame(None),
            UiStyle::Custom(frame) => WindowStyleFrame(Some(frame)),
        };
        
        if !app.is_plugin_added::<DefaultInspectorConfigPlugin>() {
            app.add_plugins(DefaultInspectorConfigPlugin);
        }
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        // if !app.is_plugin_added::<InspectSchedulePlugin>() {
        //     app.add_plugins(InspectSchedulePlugin);
        // }

        app
        .init_resource::<DebugMenuToggle>()
        .init_state::<DebugModeToggle>()
        .init_state::<DebugMenuState>()
        .insert_resource(self.keybinds_override.clone().unwrap_or_default())
        .register_type::<UiExtrasKeybinds>()
        .insert_resource(window_style)
        .init_resource::<DebugWidgetView>()
        .init_resource::<FilterResponse>()
        .init_resource::<ShowAppStatus>()
        .init_resource::<FocusOnDebugFilter>()
        .init_resource::<ComponentFilterMode>()
        //.init_resource::<SelectedEntities>()
        .register_type::<FilterResponse>()
        .add_systems(Update, debug_menu.run_if(in_state(DebugMenuState::Open)))
        .add_systems(Update, manage_debug_menu_state)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        ;
    }
}

/// Plugin for registering the given `T` for `DebugModeToggle` to enable/disable debug mod for a given debug mode toggle.
#[derive(Default)]
pub struct DebugModeRegister<T: DerefMut<Target = bool> + Resource>(pub PhantomData<T>);

impl<T: DerefMut<Target = bool> + Resource> Plugin for DebugModeRegister<T> {
    fn build(&self, app: &mut bevy_app::App) {
        app
        .add_systems(OnEnter(DebugModeToggle::On), set_entry_to_on::<T>)
        .add_systems(OnEnter(DebugModeToggle::Off), set_entry_to_off::<T>)

        ;
    }
}
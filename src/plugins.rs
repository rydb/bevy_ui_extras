use std::marker::PhantomData;
use std::ops::DerefMut;

use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
// use bevy_inspector_egui::quick::AssetInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
// use bevy_pbr::StandardMaterial;
use bevy_state::prelude::*;
// use bevy_pbr::StandardMaterial;
use bevy_app::{Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{debug_menu, states::DebugMenuState, FilterResponse, KeyBinds};
use crate::{
    display_debug_menu_explanation, manage_debug_menu_state, set_entry_to_off, set_entry_to_on,
    ComponentFilterMode, DebugMenuToggle, DebugModeFlagToggle, DebugWidgetView, FilterKind,
    FocusOnDebugFilter, ShowAppStatus, UiAlignment, UiStyle,
};

/// plugin for general debug menu. See [`KeyBinds`] for keybinds.
pub struct UiExtrasDebug {
    pub ui_style: UiStyle,
    pub alignment: UiAlignment,
    pub default_filters: Vec<FilterKind>,
    pub keybinds_override: Option<KeyBinds>,
    pub menu_mode: DebugMenuState,
}

impl Default for UiExtrasDebug {
    fn default() -> Self {
        Self {
            ui_style: UiStyle::BLACK_GLASS,
            alignment: UiAlignment::default(),
            keybinds_override: None,
            default_filters: vec![],
            menu_mode: DebugMenuState::Closed,
        }
    }
}

impl Plugin for UiExtrasDebug {
    fn build(&self, app: &mut bevy_app::App) {
        // let window_style = match self.ui_style {
        //     UiStyle::Black_Glass => UiStyle::default(),
        //     UiStyle::Default => UiStyle(None),
        //     UiStyle::Custom(frame) => UiStyle(Some(frame)),
        // };

        if !app.is_plugin_added::<DefaultInspectorConfigPlugin>() {
            app.add_plugins(DefaultInspectorConfigPlugin);
        }
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        // if !app.is_plugin_added::<AssetInspectorPlugin::<StandardMaterial>>() {
        //     app.add_plugins(AssetInspectorPlugin::<StandardMaterial>::default());
        // }

        // if !app.is_plugin_added::<InspectSchedulePlugin>() {
        //     app.add_plugins(InspectSchedulePlugin);
        // }

        app.init_resource::<DebugMenuToggle>()
            .insert_state(self.menu_mode.clone())
            .insert_state(DebugModeFlagToggle::Off)
            .insert_resource(self.keybinds_override.clone().unwrap_or_default())
            .register_type::<KeyBinds>()
            .insert_resource(self.ui_style.clone())
            .insert_resource(self.alignment.clone())
            .init_resource::<DebugWidgetView>()
            .init_resource::<FilterResponse>()
            .init_resource::<ShowAppStatus>()
            .init_resource::<FocusOnDebugFilter>()
            .init_resource::<ComponentFilterMode>()
            //.init_resource::<SelectedEntities>()
            .register_type::<FilterResponse>()
            //.register_asset_reflect::<StandardMaterial>()
            .add_systems(Update, debug_menu.run_if(in_state(DebugMenuState::Open)))
            .add_systems(Update, manage_debug_menu_state)
            .add_systems(
                Update,
                display_debug_menu_explanation.run_if(in_state(DebugMenuState::Explain)),
            )
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(SystemInformationDiagnosticsPlugin);
    }
}

/// Plugin for registering debug mode flags. If your resource is a bool newtype. implement deref into bool for it and, register it with
/// ```rust
/// app.add_plugins(DebugModeFlagRegistry::<T>::default())
/// ```
/// and then you can enable it through the debug menu.
#[derive(Default)]
pub struct DebugModeFlagRegister<T: DerefMut<Target = bool> + Resource>(pub PhantomData<T>);

impl<T: DerefMut<Target = bool> + Resource> Plugin for DebugModeFlagRegister<T> {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(OnEnter(DebugModeFlagToggle::On), set_entry_to_on::<T>)
            .add_systems(OnEnter(DebugModeFlagToggle::Off), set_entry_to_off::<T>);
    }
}

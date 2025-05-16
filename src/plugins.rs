use std::marker::PhantomData;
use std::ops::DerefMut;

use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::egui::Align2;
// use bevy_inspector_egui::quick::AssetInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
// use bevy_pbr::StandardMaterial;
use bevy_state::prelude::*;
// use bevy_pbr::StandardMaterial;
use bevy_app::{Plugin, PluginGroup, PluginGroupBuilder, Update};
use bevy_ecs::prelude::*;

use crate::widgets::app_status::plugins::AppStatusMenuPlugin;
use crate::widgets::debug_menu::plugins::DebugMenuPlugin;
use crate::widgets::debug_menu::{DebugMenuKeybinds, DebugMenuState};
use crate::{
    Opacity, UiAlignment, UiStyle,
};





/// setup plugin for [`UiExtrasDebug`].
pub struct UiExtrasDebugSetup {
    pub ui_style: UiStyle,
    pub alignment: Option<Align2>,
    // pub default_filters: Vec<FilterKind>,
    pub keybinds_override: Option<DebugMenuKeybinds>,
    pub menu_mode: DebugMenuState,
}

impl Default for UiExtrasDebugSetup {
    fn default() -> Self {
        Self {
            ui_style: UiStyle::BLACK_GLASS,
            alignment: None,
            keybinds_override: None,
            // default_filters: vec![],
            menu_mode: DebugMenuState::Closed,
        }
    }
}

impl Plugin for UiExtrasDebugSetup {
    fn build(&self, app: &mut bevy_app::App) {;
        if !app.is_plugin_added::<DefaultInspectorConfigPlugin>() {
            app.add_plugins(DefaultInspectorConfigPlugin);
        }
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin {
                //(TODO): Setting this to false because it causes a panic
                //once this is fixed, set this to true.
                enable_multipass_for_primary_context: false,
            });
        }
        // if !app.is_plugin_added::<AssetInspectorPlugin::<StandardMaterial>>() {
        //     app.add_plugins(AssetInspectorPlugin::<StandardMaterial>::default());
        // }

        // if !app.is_plugin_added::<InspectSchedulePlugin>() {
        //     app.add_plugins(InspectSchedulePlugin);
        // }

        let opacity = match self.ui_style.0 {
            Some(style) => style.fill.a(),
            None => u8::MAX,
        };
        app
            .insert_state(self.menu_mode.clone())
            .insert_resource(self.keybinds_override.clone().unwrap_or_default())
            .insert_resource(Opacity(opacity))
            .insert_resource(self.ui_style.clone())
            .insert_resource(UiAlignment(self.alignment.clone()))
        ;

    }
}


/// plugin for general debug menu. See [`KeyBinds`] for keybinds.
pub struct UiExtrasDebug(pub UiExtrasDebugSetup);

impl PluginGroup for UiExtrasDebug {
    fn build(self) -> bevy_app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(self.0)
        .add(DebugMenuPlugin)
        .add(AppStatusMenuPlugin)
    }
}
use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy_state::prelude::*;
use bevy_app::{Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{manage_debug_menu_state, ComponentFilterMode, FocusOnDebugFilter, ShowAppStatus, UiStyle, WindowStyleFrame};
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
        
        app
        .init_state::<DebugMenuState>()
        .insert_resource(self.keybinds_override.clone().unwrap_or_default())
        .register_type::<UiExtrasKeybinds>()
        .insert_resource(window_style)
        .init_resource::<FilterResponse>()
        .init_resource::<ShowAppStatus>()
        .init_resource::<FocusOnDebugFilter>()
        .init_resource::<ComponentFilterMode>()
        .register_type::<FilterResponse>()
        .add_systems(Update, debug_menu.run_if(in_state(DebugMenuState::Open)))
        .add_systems(Update, manage_debug_menu_state)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        ;
    }
}
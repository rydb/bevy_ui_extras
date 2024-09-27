use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy_state::prelude::*;
use bevy_app::{Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{manage_debug_menu_state, FocusOnDebugFilter, ShowAppStatus};
use crate::{debug_menu, states::DebugMenuState, FilterResponse, UiExtrasKeybinds};

/// plugin for general debugging 
pub struct UiExtrasDebug;

impl Plugin for UiExtrasDebug {
    fn build(&self, app: &mut bevy_app::App) {
        app
        .init_state::<DebugMenuState>()
        .init_resource::<UiExtrasKeybinds>()
        .register_type::<UiExtrasKeybinds>()
        .init_resource::<FilterResponse>()
        .init_resource::<ShowAppStatus>()
        .init_resource::<FocusOnDebugFilter>()
        //.init_resource::<SelectedComponentsUi>()
        .register_type::<FilterResponse>()
        //.register_type::<Pane>()
        .add_systems(Update, debug_menu.run_if(in_state(DebugMenuState::Open)))
        .add_systems(Update, manage_debug_menu_state)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)
        //.add_plugins(Diag)
        ;
    }
}
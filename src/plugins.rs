use bevy_app::{Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{debug_menu, display_selected_components, FilterResponse, Pane, SelectedComponentsUi, UiExtrasKeybinds};

/// plugin for general debugging 
pub struct UiExtrasDebug;

impl Plugin for UiExtrasDebug {
    fn build(&self, app: &mut bevy_app::App) {
        app
        .init_resource::<UiExtrasKeybinds>()
        .init_resource::<FilterResponse>()
        //.init_resource::<SelectedComponentsUi>()
        .register_type::<FilterResponse>()
        .register_type::<Pane>()
        .add_systems(Update, debug_menu)
        .add_systems(Update, display_selected_components)
        ;
    }
}
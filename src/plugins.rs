use bevy_app::{Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{debug_menu, FilterResponse, UiExtrasKeybinds};

/// plugin for general debugging 
pub struct UiExtrasDebug;

impl Plugin for UiExtrasDebug {
    fn build(&self, app: &mut bevy_app::App) {
        app
        .init_resource::<UiExtrasKeybinds>()
        .init_resource::<FilterResponse>()
        .register_type::<FilterResponse>()
        .add_systems(Update, debug_menu)
        ;
    }
}
use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_state::prelude::*;
use bevy_derive::DerefMut;
use bevy_ecs::resource::Resource;
use std::ops::DerefMut;

use crate::{set_entry_to_off, set_entry_to_on};

use super::{systems::*, resources::*, *};


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

pub struct DebugMenuPlugin;

impl Plugin for DebugMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            display_debug_menu_explanation.run_if(in_state(DebugMenuState::Explain)),
        )
        .register_type::<FilterResponse>()
        .init_resource::<FocusOnDebugFilter>()
        .init_resource::<FilterResponse>()
        .insert_state(DebugModeFlagToggle::Off)
        .add_systems(Update, debug_menu.run_if(in_state(DebugMenuState::Open)))
        .add_systems(Update, manage_debug_menu_state)
        .init_resource::<DebugMenuToggle>()
        .init_resource::<DebugWidgetView>()
        .init_resource::<ComponentFilterMode>()
        .register_type::<DebugMenuKeybinds>()



        ;
    }
}
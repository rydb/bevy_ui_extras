use std::{collections::BTreeSet, ops::DerefMut};

use bevy_ecs::resource::Resource;
use bevy_reflect::prelude::*;
use bevy_state::state::States;
use bevy_ecs::prelude::ReflectResource;
use bevy_app::prelude::*;
use bevy_input::prelude::*;
use resources::DebugMenuToggle;

pub mod resources;
pub mod plugins;
mod systems;


pub trait DebugTarget: DerefMut<Target = bool> + Resource {}


#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum DebugMenuState {
    /// Explain controls to manage debug menu
    Explain,
    Open,
    #[default]
    Closed,
}

/// Keybinds for this crate.
#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct DebugMenuKeybinds {
    /// keyibnd to toggle debug menu on and off.
    pub toggle_debug_menu: KeyCode,
    /// keybind to quickly open the debug menu and filter for specific components/resources
    pub filter_quick_focus: BTreeSet<KeyCode>,
    /// clears all selected values in debug menu
    pub clear: BTreeSet<KeyCode>,
    /// cycles between different debug view modes
    pub cycle_views: KeyCode,
}

impl Default for DebugMenuKeybinds {
    fn default() -> Self {
        let mut x = DebugMenuToggle(false);
        *x ^= true;

        Self {
            toggle_debug_menu: KeyCode::Backquote,
            filter_quick_focus: [KeyCode::ControlLeft, KeyCode::KeyF].into(),
            clear: [KeyCode::ControlLeft, KeyCode::KeyC].into(),
            cycle_views: KeyCode::AltLeft,
        }
    }
}

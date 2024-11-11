
use std::{any::TypeId, collections::BTreeSet, fmt::Debug, ops::DerefMut};

use bevy_state::prelude::*;
use bevy_app::Plugin;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_input::prelude::KeyCode;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;
use egui::Frame;
use strum_macros::{Display, EnumIter};

use crate::{stylesheets::DEBUG_FRAME_STYLE, TypeIdNameCache};

/// Toggle to enable debug mode resources
#[derive(Resource, Deref, DerefMut, Default)]
pub struct DebugMenuToggle(pub bool);

#[derive(Resource, Default, EnumIter, Display, PartialEq, Eq)]
pub enum ComponentFilterMode {
    AND,
    #[default]
    OR,
}

// #[derive(Resource, Default, Clone)]
// pub struct SelectedEntities(pub BTreeSet<Entity>);

#[derive(Resource, Default, EnumIter, Display, PartialEq, Eq, Clone)]
pub enum DebugWidgetView {
    EntitiesView,
    #[default]
    ComponentsView,
}

// pubs struct ComponentFilterMode()

/// the style that egui windows for this library use. See [`stylesheets.rs`] for what those look like.
#[derive(Resource)]
pub struct WindowStyleFrame(pub Option<Frame>);

impl Default for WindowStyleFrame {
    fn default() -> Self {
        Self(Some(DEBUG_FRAME_STYLE))
    }
}
#[derive(Resource)]
pub struct ShowAppStatus(pub bool);
impl Default for ShowAppStatus {
    fn default() -> Self {
        Self(false)
    }
}
#[derive(Resource)]
pub(crate) struct FocusOnDebugFilter(pub bool);

impl Default for FocusOnDebugFilter {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Resource, Reflect, Default, Clone)]
pub struct FilterResponse {
    pub filter: String,
    pub selected_type: HashMap<TypeId, TypeIdNameCache>,
    //pub fuzzy_match_enabled: bool,
}

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct UiExtrasKeybinds {
    /// keyibnd to toggle debug menu on and off. 
    pub toggle_debug_menu: KeyCode,
    /// keybind to quickly open the debug menu and filter for specific components/resources
    pub filter_quick_focus: BTreeSet<KeyCode>,
    /// clears all selected values in debug menu
    pub clear: BTreeSet<KeyCode>,
    /// cycles between different debug view modes
    pub cycle_views: KeyCode
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Clone)]
pub enum DebugModeToggle {
    On,
    #[default]
    Off,
}

// pub trait bob = Default + DerefMut;

pub trait DebugTarget: DerefMut<Target = bool> + Resource {}


pub struct DebugModeRegistry<T: DerefMut<Target = bool> + Resource>(T);

impl<T: DerefMut<Target = bool> + Resource> Plugin for DebugModeRegistry<T> {
    fn build(&self, app: &mut bevy_app::App) {
        app
        .add_systems(OnTransition {entered: DebugModeToggle::On, exited: DebugModeToggle::Off}, set_entry_to_toggle::<T>)
        ;
    }
}

fn set_entry_to_toggle<T: DerefMut<Target = bool> + Resource>(
    mut debug_mode_target: ResMut<T>,
    debug_mode_toggle: Res<State<DebugModeToggle>>
) {
    match **debug_mode_toggle {
        DebugModeToggle::On => **debug_mode_target = true,
        DebugModeToggle::Off => **debug_mode_target = false,
    }
}

impl Default for UiExtrasKeybinds {

    fn default() -> Self {
        let mut x = DebugMenuToggle(false);
        *x ^= true;

        Self {
            toggle_debug_menu: KeyCode::Backquote,
            filter_quick_focus: [KeyCode::ControlLeft, KeyCode::KeyF].into(),
            clear: [KeyCode::ControlLeft, KeyCode::KeyC].into(),
            cycle_views: KeyCode::AltLeft
        }
    }
}

use std::{any::TypeId, collections::BTreeSet};

use bevy_ecs::prelude::*;
use bevy_input::prelude::KeyCode;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;
use egui::Frame;
use strum_macros::{Display, EnumIter};

use crate::{stylesheets::DEBUG_FRAME_STYLE, TypeIdNameCache};

#[derive(Resource, Default, EnumIter, Display, PartialEq, Eq)]
pub enum ComponentFilterMode {
    AND,
    #[default]
    OR,
}

// pubs struct ComponentFilterMode()

/// the style that egui windows for this library use. See [`stylesheets.rs`] for what those look like.
#[derive(Resource)]
pub struct WindowStyleFrame {
    pub frame: Frame,
}

impl Default for WindowStyleFrame {
    fn default() -> Self {
        Self {
            frame: DEBUG_FRAME_STYLE,
        }
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

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct UiExtrasKeybinds {
    /// keyibnd to toggle debug menu on and off. 
    pub toggle_debug_menu: KeyCode,
    /// keybind to quickly open the debug menu and filter for specific components/resources
    pub filter_quick_focus: BTreeSet<KeyCode>,
    /// clears all selected values in debug menu
    pub clear: BTreeSet<KeyCode>
}

#[derive(Resource)]
pub struct DebugMenuToggle(pub bool);

impl Default for UiExtrasKeybinds {
    fn default() -> Self {
        Self {
            toggle_debug_menu: KeyCode::AltLeft,
            filter_quick_focus: [KeyCode::ControlLeft, KeyCode::KeyF].into(),
            clear: [KeyCode::ControlLeft, KeyCode::KeyC].into(),
        }
    }
}
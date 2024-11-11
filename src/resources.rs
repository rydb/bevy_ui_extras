
use std::{any::TypeId, cell::Cell, collections::BTreeSet, fmt::Debug, ops::DerefMut, sync::Arc};

use bevy_app::Plugin;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::ComponentId, prelude::*};
use bevy_input::prelude::KeyCode;
use bevy_ecs::system::Resource;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;
use egui::Frame;
use strum_macros::{Display, EnumIter};

use crate::{stylesheets::DEBUG_FRAME_STYLE, TypeIdNameCache};

/// Toggle to enable debug mode resources
pub struct DebugModeToggle(pub bool);

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

#[derive(Resource, Default, Deref, DerefMut)]
pub struct DebugMenuToggle(pub bool);

// pub trait bob = Default + DerefMut;

pub trait DebugTarget: DerefMut<Target = bool> + Resource {}


pub struct DebugModeToggleRegistry{
    pub registery: HashMap<String, ComponentId>
}


// impl DebugModeToggleRegistry{
//     // pub fn add<T: DerefMut<Target = bool> + Resource>(&mut self, item: T) -> self {
        
//     //     self.registery.insert(format!("{:#?}", item), Box::new(item))
//     // }
//     pub fn flip(&self, world: &mut World) {
//         let (name, item) = self.registery.iter().last().unwrap();
        
//         let registration = type_registry
//             .get(type_id)
//             .ok_or(Error::NoTypeRegistration(type_id))?;
//         let reflect_from_ptr = registration
//             .data::<ReflectFromPtr>()
//             .ok_or(Error::NoTypeData(type_id, "ReflectFromPtr"))?;

//         let (ptr, set_changed) = crate::utils::mut_untyped_split(value);
//         assert_eq!(reflect_from_ptr.type_id(), type_id);
//         // SAFETY: ptr is of type type_id as required in safety contract, type_id was checked above
//         let value = unsafe { reflect_from_ptr.as_reflect_mut(ptr) };

//         Ok((value, set_changed))
//     }
// }


// impl Plugin for DebugModeToggleRegistry {
//     fn build(&self, app: &mut bevy_app::App) {
        
//     }
// }

// pub fn set_debug_mode<T: Component + DerefMut<Target = bool>>(
//     mut debug_state_registry: ResMut<DebugModeToggleRegistry>,
//     //target: Res<T>,
// ) {
//     for (name, target) in debug_state_registry.registery.iter_mut() {
//         ***target ^= true;
//     }
// }

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
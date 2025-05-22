use std::{any::TypeId, collections::HashMap, ops::Deref};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::resource::Resource;
use bevy_reflect::Reflect;
use bevy_state::state::States;
use strum_macros::{Display, EnumIter};

use crate::{TypeIdNameCache};

#[derive(Resource)]
pub(crate) struct FocusOnDebugFilter(pub bool);

impl Default for FocusOnDebugFilter {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Resource, Reflect, Default, Clone)]
pub struct FilterResponse {
    pub filter_prompt: String,
    pub filters: Vec<FilterKind>,
    pub selected_type: HashMap<TypeId, TypeIdNameCache>,
    //pub fuzzy_match_enabled: bool,
}

/// (W.I.P) What components are filtered by
#[derive(Reflect, Clone)]
pub enum FilterKind {
    Crate(String),
    Name(String),
}



/// Resource that toggles all resources that are toggled
#[derive(States, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Clone)]
pub enum DebugModeFlagToggle {
    On,
    #[default]
    Off,
}


/// Toggle to enable debug mode resources
#[derive(Resource, Deref, DerefMut, Default)]
pub struct DebugMenuToggle(pub bool);


// #[derive(Resource, Default, Clone)]
// pub struct SelectedEntities(pub BTreeSet<Entity>);

#[derive(Resource, Default, EnumIter, Display, PartialEq, Eq, Clone)]
pub enum DebugWidgetView {
    EntitiesView,
    #[default]
    ComponentsView,
}

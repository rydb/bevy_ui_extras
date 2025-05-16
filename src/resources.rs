use std::{
    any::TypeId,
    collections::{BTreeSet, HashMap},
    fmt::Debug,
    ops::{DerefMut, RangeInclusive},
};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_input::prelude::KeyCode;
use bevy_inspector_egui::egui::{Align2, Frame};
use bevy_reflect::Reflect;
use bevy_state::prelude::*;
use strum_macros::{Display, EnumIter};

use crate::{TypeIdNameCache, stylesheets::DEBUG_FRAME_STYLE};


// pub struct Opacity(pub RangeInclusive<{ 0..100 }>);

pub const OPACITY_RANGE: RangeInclusive<u8> = RangeInclusive::new(0, 255);

#[derive(Resource, Default, Clone)]
pub struct Opacity(pub u8);

/// debug menu styles
#[derive(Resource, Default, Clone)]
pub struct UiStyle(pub Option<Frame>);

#[derive(Resource, Clone)]
pub struct UiAlignment(pub Option<Align2>);

impl Default for UiAlignment {
    fn default() -> Self {
        Self(Some(Align2::LEFT_TOP))
    }
}

impl UiStyle {
    //TODO: Add more styles here.
    pub const BLACK_GLASS: Self = Self(Some(DEBUG_FRAME_STYLE));
}


// pub trait bob = Default + DerefMut;



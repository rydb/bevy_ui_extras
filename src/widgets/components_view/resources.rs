use bevy_ecs::prelude::*;
use strum_macros::{Display, EnumIter};


#[derive(Resource, Default, EnumIter, Display, PartialEq, Eq)]
pub enum ComponentFilterMode {
    AND,
    #[default]
    OR,
}

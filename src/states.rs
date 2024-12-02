use bevy_state::prelude::*;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum DebugMenuState {
    Open,
    #[default]
    Closed,
}

use bevy_state::prelude::*;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum DebugMenuState {
    /// Explain controls to manage debug menu
    Explain,
    Open,
    #[default]
    Closed,
}

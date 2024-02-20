use bevy_ecs::system::Resource;
use egui::Frame;

use crate::stylesheets::DEBUG_FRAME_STYLE;

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

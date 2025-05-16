
use bevy_app::prelude::*;
use bevy_diagnostic::{FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};

use super::resources::ShowAppStatus;

pub struct AppStatusMenuPlugin;

impl Plugin for AppStatusMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(SystemInformationDiagnosticsPlugin)
        .init_resource::<ShowAppStatus>()
        ;
    }
}
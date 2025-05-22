use bevy_app::prelude::*;

use super::resources::*;

pub struct ComponentsView;



impl Plugin for ComponentsView {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ComponentFilterMode>()
        ;
    }
}
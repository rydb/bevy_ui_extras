
use std::any::TypeId;

use crate::TypeIdNameCache;

use super::resources::*;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;

use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, TypePath};
use bevy_utils::HashMap;
use bevy_window::{prelude::*, PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self, ScrollArea},
};
//use derive_more::From;
use egui_tiles::{SimplificationOptions, Tile, TileId, Tree};

// #[derive(Resource, Deref, DerefMut)]
// pub struct WgslInUi(Tree<Pane>);

#[derive(Resource, Default, Reflect, Deref, DerefMut)]
pub struct SelectedComponentsUi(pub HashMap<TypeId, TypeId>);


// pub type TestUi = TreeUi<Pane>;

// impl<T> Default for TreeUi<T> {
//     fn default() -> Self {
//         Self(Tree::empty(egui::Id::new(rand::random::<u64>())))
//     }
// }
// impl Default for SelectedComponentsUi {
//     fn default() -> Self {
//         Self()
//         //Self(Vec::new())
            
//             //0: Tree::<Pane>::empty("SelectedComponents")
        
//     }
// }

pub struct TreeBehavior {}

#[derive(Reflect, Debug, Default, Clone)]
pub struct Pane {
    pub name: String,
    pub component_id: Option<TypeIdNameCache>,
}

/// bind a tree ui item to a resource.
pub fn bind_tree<T: From<Tree<Pane>> + Resource>() -> T {
    let tiles = egui_tiles::Tiles::default();
    T::from(egui_tiles::Tree::new(
        "my_tree",
        TileId::from_u64(9999),
        tiles,
    ))
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.name.clone().into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        egui::ScrollArea::both().show(ui, |ui| {
            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job =
                    egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "rust");
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };
            egui::ScrollArea::vertical().show(ui, |ui| {
                
                // ui.add(
                //     egui::TextEdit::multiline(&mut pane.content)
                //         .font(egui::TextStyle::Monospace) // for cursor height
                //         .code_editor()
                //         .desired_rows(10)
                //         .lock_focus(true)
                //         .desired_width(f32::INFINITY)
                //         .layouter(&mut layouter),
                // );
            });
        });
        egui_tiles::UiResponse::None
    }
    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        // tree keeps getting deleted so all of this gets set to fault for the time being.
        SimplificationOptions {
            prune_empty_containers: false,
            prune_empty_tabs: false,
            prune_single_child_containers: false,
            prune_single_child_tabs: false,
            all_panes_must_have_tabs: false,
            join_nested_linear_containers: false,
        }
    }
    // fn min_size(&self) -> f32 {
    //     0.0
    // }
}


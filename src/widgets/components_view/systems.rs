use bevy_inspector_egui::{bevy_egui::EguiContext, egui::{self, Color32, RichText}};
use bevy_ecs::prelude::*;
use bevy_log::warn;

use super::resources::ComponentFilterMode;


#[derive(Component)]
pub struct FocusWindow;

// pub fn display_components_widget(
//     focus_window: Query<&mut EguiContext, With<FocusWindow>>
// ) {

//     egui::SidePanel::left("Components")
//     .frame(window_style)
//     .show_inside(ui, |ui| {
//         let screen_size = ui.ctx().screen_rect().size();
//         ui.set_max_size(screen_size);

//         egui::ScrollArea::new(true).show(ui, |ui| {
//             ui.horizontal(|ui| {
//                 ui.vertical(|ui| {
//                     ui.heading("Components");

//                     let Some(mut match_mode) =
//                         world.get_resource_mut::<ComponentFilterMode>()
//                     else {
//                         warn!("ComponentFilterMode doesn't exist. Aborting");
//                         return;
//                     };
//                     egui::Frame::default()
//                         .stroke(Stroke::new(2.0, Color32::BLACK))
//                         .outer_margin(5.0)
//                         .inner_margin(5.0)
//                         .show(ui, |ui| {
//                             ui.horizontal(|ui| {
//                                 for variant in ComponentFilterMode::iter() {
//                                     let color = match *match_mode == variant {
//                                         true => Color32::WHITE,
//                                         false => Color32::GRAY,
//                                     };

//                                     if ui
//                                         .button(
//                                             RichText::new(variant.to_string())
//                                                 .color(color),
//                                         )
//                                         .clicked()
//                                     {
//                                         *match_mode = variant
//                                     }
//                                 }
//                             });
//                         });
//                     let mut alphabetized_components =
//                         components_filtered.iter().collect::<Vec<_>>();

//                     alphabetized_components
//                         .sort_by(|(_, (name, _)), (_, (name2, _))| name.cmp(name2));
//                     for (id, (name, origin)) in alphabetized_components.iter() {
//                         let color = match debug_filter_response
//                             .selected_type
//                             .contains_key(*id)
//                         {
//                             true => Color32::WHITE,
//                             false => Color32::GRAY,
//                         };
//                         let button = ui.button(RichText::new(*name).color(color));

//                         if button.clicked() {
//                             let Some(mut debug_filter_response) =
//                                 world.get_resource_mut::<FilterResponse>()
//                             else {
//                                 warn!("FilterResponse doesn't exist. Aborting");
//                                 return;
//                             };
//                             let type_id_cache = TypeIdNameCache {
//                                 type_id: **id,
//                                 name: (**name).to_owned(),
//                             };

//                             if debug_filter_response
//                                 .selected_type
//                                 .get(*id)
//                                 .is_some()
//                             {
//                                 if multi_select == false {
//                                     debug_filter_response.selected_type.clear();
//                                 } else {
//                                     debug_filter_response.selected_type.remove(*id);
//                                 }
//                             } else {
//                                 if multi_select == false {
//                                     debug_filter_response.selected_type.clear();
//                                 }
//                                 debug_filter_response
//                                     .selected_type
//                                     .insert(**id, type_id_cache);
//                             }
//                         };
//                         if button.hovered() {
//                             ui.label(*origin);
//                         }
//                     }
//                 });
//             });
//         });
//     });

// }
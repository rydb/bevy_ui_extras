use std::any::*;

use bevy_ecs::component::ComponentId;
use bevy_ecs::prelude::*;
use bevy_ecs::world::CommandQueue;
use bevy_inspector_egui::egui::{self, Frame, Ui};
use bevy_inspector_egui::reflect_inspector::{Context, InspectorUi};
use bevy_inspector_egui::restricted_world_view::RestrictedWorldView;
use bevy_log::warn;
use bevy_reflect::*;
use std::hash::Hash;

pub mod components;
pub mod plugins;
pub mod resources;
pub mod stylesheets;
pub mod systems;
pub mod tables;
pub mod widgets;
// pub mod tree;
pub mod states;

pub enum Display {
    Side(Side),
    Window,
}

pub enum Side {
    Left,
    Right,
}

pub use components::*;
pub use plugins::*;
pub use resources::*;
pub use stylesheets::*;
pub use systems::*;
// pub use tables::*;
// pub use tree::*;

/// helper struct for keeping name and type_id together after type erasing a type.
#[derive(Reflect, Clone, PartialEq, Hash, Eq, Debug)]
pub struct TypeIdNameCache {
    pub(crate) type_id: TypeId,
    pub(crate) name: String,
}

impl TypeIdNameCache {
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    /// create this from a typed T
    pub fn new_typed<T: Reflect>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            name: std::any::type_name::<T>().to_owned(),
        }
    }
}



/// renders ui for a given resource.
pub fn ui_for_resource(
    world: &mut World,
    ui: &mut egui::Ui,
    id: egui::Id,
    type_registry: &TypeRegistry,
    resource: &TypeIdNameCache,
) {
    let mut queue = CommandQueue::default();

    let resource_type_id = resource.type_id();
    let id = id.with(resource_type_id);

    {
        // create a context with access to the world except for the current resource
        let mut world_view = RestrictedWorldView::new(world);
        let (mut resource_view, world_view) = world_view.split_off_resource(resource_type_id);
        let mut cx = Context {
            world: Some(world_view),
            queue: Some(&mut queue),
        };
        let mut env = InspectorUi::for_bevy(type_registry, &mut cx);

        let mut resource =
            match resource_view.get_resource_reflect_mut_by_id(resource_type_id, type_registry) {
                Ok(resource) => resource,
                Err(err) => {
                    ui.label(format!(
                        "unable to display: {:#?}, Reason: {:#?}",
                        resource_type_id, err
                    ));
                    return;
                } //return errors::show_error(err, ui, name_of_type),
            };

        let changed = env.ui_for_reflect_with_options(
            resource.bypass_change_detection().as_partial_reflect_mut(),
            ui,
            id.with(resource_type_id),
            &(),
        );
        if changed {
            resource.set_changed();
        }
    }

    queue.apply(world);
}



// Display the ui for a componenent of an entity
// pub fn ui_for_entity_components<T: Component>(
//     world: &mut RestrictedWorldView<'_>,
//     mut queue: Option<&mut CommandQueue>,
//     entity: Entity,
//     ui: &mut egui::Ui,
//     id: egui::Id,
//     type_registry: &TypeRegistry,
// ) {

//     let Some(components) = components_of_entity(world, entity) else {
//         //errors::entity_does_not_exist(ui, entity);
//         return;
//     };

//     for (name, component_id, component_type_id, size) in components {
//         let id = id.with(component_id);

//         let header = egui::CollapsingHeader::new(&name).id_source(id);

//         let Some(component_type_id) = component_type_id else {
//             //header.show(ui, |ui| errors::no_type_id(ui, &name));
//             continue;
//         };

//         if size == 0 {
//             header.show(ui, |_| {});
//             continue;
//         }

//         // create a context with access to the world except for the currently viewed component
//         let (mut component_view, world) = world.split_off_component((entity, component_type_id));
//         let mut cx = Context {
//             world: Some(world),
//             #[allow(clippy::needless_option_as_deref)]
//             queue: queue.as_deref_mut(),
//         };

//         let (value, is_changed, set_changed) = match component_view.get_entity_component_reflect(
//             entity,
//             component_type_id,
//             type_registry,
//         ) {
//             Ok(value) => value,
//             Err(e) => {
//                 //header.show(ui, |ui| errors::show_error(e, ui, &name));
//                 continue;
//             }
//         };

//         if is_changed {
//             #[cfg(feature = "highlight_changes")]
//             set_highlight_style(ui);
//         }

//         header.show(ui, |ui| {
//             ui.reset_style();

//             let inspector_changed = InspectorUi::for_bevy(type_registry, &mut cx)
//                 .ui_for_reflect_with_options(value, ui, id.with(component_id), &());

//             if inspector_changed {
//                 set_changed();
//             }
//         });
//         ui.reset_style();
//     }
// }

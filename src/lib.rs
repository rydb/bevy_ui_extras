use std::any::*;

use bevy_ecs::prelude::*;
use bevy_ecs::component::ComponentId;
use bevy_ecs::query::QueryFilter;
use bevy_ecs::world::CommandQueue;
use bevy_inspector_egui::reflect_inspector::{Context, InspectorUi};
use bevy_inspector_egui::restricted_world_view::RestrictedWorldView;
use bevy_log::warn;
use bevy_reflect::*;

pub mod components;
pub mod resources;
pub mod stylesheets;
pub mod systems;
pub mod tables;
pub mod plugins;
pub mod tree;
pub enum Display {
    Side(Side),
    Window,
}

pub enum Side {
    Left,
    Right,
}

use bevy_utils::HashMap;
pub use plugins::*;
pub use components::*;
pub use resources::*;
pub use stylesheets::*;
pub use systems::*;
pub use tables::*;
pub use tree::*;


/// helper struct for keeping name and type_id together after type erasing a type.
#[derive(Reflect, Clone, Debug)]
pub struct TypeIdNameCache {
    pub(crate) type_id: TypeId,
    pub(crate) name:  String,
}

impl TypeIdNameCache {
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    /// create this from a typed T
    pub fn new_typed<T: Reflect>() -> Self{
        Self {
            type_id: TypeId::of::<T>(),
            name: std::any::type_name::<T>().to_owned()
        }
    }
    //pub(crate) fn new_untyped(type_id: TypeId, name:)
}


/// fetches the info for the componenet of type T for the given entity, if it exists. 
pub fn component_info_for(
    world: &mut RestrictedWorldView<'_>,
    component: &TypeIdNameCache
    //entity: Entity,
) -> Option<(String, ComponentId, Option<TypeId>, usize)> {
    
    //let entity_ref = world.world().get_entity(entity)?;

    let component_id  = match world.world().components().get_id(component.type_id) {
        Some(id) => id,
        None => {
            warn!("Could not get component id for {:#}", component.name);
            return None
        }
    };
    //let archetype = entity_ref.archetype();
    
    let info = world.world().components().get_info(component_id)?;
    
    let name = pretty_type_name::pretty_type_name_str(info.name());

    return Some(
        (
            name,
            component_id,
            info.type_id(),
            info.layout().size()
        )

    );
}

pub fn ui_for_resource(
    world: &mut World,
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    resource: &TypeIdNameCache,
) {
    let mut queue = CommandQueue::default();

    //let resource = TypeIdNameCache::t
    let resource_type_id = resource.type_id();
    {
        // create a context with access to the world except for the current resource
        let mut world_view = RestrictedWorldView::new(world);
        let (mut resource_view, world_view) = world_view.split_off_resource(resource_type_id);
        let mut cx = Context {
            world: Some(world_view),
            queue: Some(&mut queue),
        };
        let mut env = InspectorUi::for_bevy(type_registry, &mut cx);

        let (resource, set_changed) = match resource_view
            .get_resource_reflect_mut_by_id(resource_type_id, type_registry)
        {
            Ok(resource) => resource,
            Err(err) => {
                ui.label(format!("unable to display: {:#?}, Reason: {:#?}", resource_type_id, err));
                return;
            },//return errors::show_error(err, ui, name_of_type),
        };

        let changed = env.ui_for_reflect(resource, ui);
        if changed {
            set_changed();
        }
    }

    

    queue.apply(world);
    // for (name, type_id) in resources {
    //     ui.collapsing(name, |ui| {
    //         by_type_id::ui_for_resource(world, type_id, ui, name, &type_registry);
    //     });
    // }
}

pub fn ui_for_components(world: &mut World, type_registry: &TypeRegistry, ui: &mut egui::Ui, entities: Vec<Entity>, component: &TypeIdNameCache) {
    //let entities_test = world.query_filtered::<Entity, With<QueryFilter
    let mut queue = CommandQueue::default();
    //let component = TypeIdNameCache::new_typed::<T>();
    let name = component.name();

    for entity in entities {

        ui.label(name);

        ui_for_component(
            &mut world.into(),
            Some(&mut queue),
            entity.clone(),
            ui,
            egui::Id::new(entity),
            &type_registry,
            component
        );
    }

    queue.apply(world);
}


pub fn ui_for_component(
    world: &mut RestrictedWorldView<'_>,
    mut queue: Option<&mut CommandQueue>,
    entity: Entity,
    ui: &mut egui::Ui,
    id: egui::Id,
    type_registry: &TypeRegistry,
    component: &TypeIdNameCache,
) {
    //let component = TypeIdNameCache::new_typed::<T>();
    let Some((name, component_id, component_type_id, size)) = component_info_for(world, component) else {return;};
    
    let id = id.with(component_id);

    let header = egui::CollapsingHeader::new(&name).id_source(id).default_open(true);

    let Some(component_type_id) = component_type_id else {return;};

    if size == 0 {
        header.show(ui, |_| {});
        return;
    }

    // create a context with access to the world except for the currently viewed component
    let (mut component_view, world) = world.split_off_component((entity, component_type_id));
    let mut cx = Context {
        world: Some(world),
        #[allow(clippy::needless_option_as_deref)]
        queue: queue.as_deref_mut(),
    };

    let (value, is_changed, set_changed) = match component_view.get_entity_component_reflect(
        entity,
        component_type_id,
        type_registry,
    ) {
        Ok(value) => value,
        Err(..) => {
            //header.show(ui, |ui| errors::show_error(e, ui, &name));
            return;
        }
    };

    if is_changed {
        #[cfg(feature = "highlight_changes")]
        set_highlight_style(ui);
    }

    header.show(ui, |ui| {
        ui.reset_style();

        let inspector_changed = InspectorUi::for_bevy(type_registry, &mut cx)
            .ui_for_reflect_with_options(value, ui, id.with(component_id), &());

        if inspector_changed {
            set_changed();
        }
    });
    ui.reset_style();
    
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
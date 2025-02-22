use std::collections::HashMap;

use fmc::{items::ItemId, prelude::*};

mod dropped_items;

pub use dropped_items::DroppedItem;

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemRegistry::default())
            .add_plugins(dropped_items::DroppedItemsPlugin);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemUseSystems;

// TODO: Transfer this over to fmc lib and have the entity be part of the ItemConfig of the item
// that can be used?
#[derive(Resource, Deref, DerefMut, Default)]
pub struct ItemRegistry(HashMap<ItemId, Entity>);

// List of player entities that have used the item during the last tick.
#[derive(Component, Default)]
pub struct ItemUses(Vec<Entity>);

impl ItemUses {
    fn read(&mut self) -> impl Iterator<Item = Entity> + '_ {
        self.0.drain(..)
    }

    pub fn push(&mut self, player_entity: Entity) {
        self.0.push(player_entity);
    }
}

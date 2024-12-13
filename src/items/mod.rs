use std::collections::HashMap;

use fmc::{items::ItemId, prelude::*};

pub mod crafting;
mod ground_items;

mod bread;
mod hoes;
mod seeds;

pub use ground_items::GroundItemBundle;

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UsableItems::default())
            .add_plugins(ground_items::GroundItemPlugin)
            .add_plugins(crafting::CraftingPlugin)
            .add_plugins(hoes::HoePlugin)
            .add_plugins(seeds::SeedPlugin);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegisterItemUse;

// TODO: Transfer this over to fmc lib and have the entity be part of the ItemConfig of the item
// that can be used.
#[derive(Resource, Deref, DerefMut, Default)]
pub struct UsableItems(HashMap<ItemId, Entity>);

// TODO: Some items might be able to interact with multiple types of blocks. Having one
// component hold all uses makes it so you have to handle all of them in one system.
// A better approach might be to register relationships, for example, ("hoe": "dirt") and
// ("hoe": "wheat") and have these be separate entities with marker components.
//
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

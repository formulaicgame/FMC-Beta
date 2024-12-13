use fmc::{
    blocks::{BlockId, Blocks},
    items::Items,
    players::Player,
    prelude::*,
};

use crate::players::Inventory;

use super::{ItemUses, UsableItems};

pub struct BreadPlugin;
impl Plugin for BreadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_bread)
            .add_systems(Update, eat_bread.after(super::RegisterItemUse));
    }
}

#[derive(Component)]
struct Bread(BlockId);

fn register_bread(
    mut commands: Commands,
    blocks: Res<Blocks>,
    items: Res<Items>,
    mut usable_items: ResMut<UsableItems>,
) {
    usable_items.insert(
        items.get_id("bread").unwrap(),
        commands
            .spawn((ItemUses::default(), Bread(blocks.get_id("bread"))))
            .id(),
    );
}

fn eat_bread(
    mut bread_uses: Query<&mut ItemUses, (With<Bread>, Changed<ItemUses>)>,
    mut player_query: Query<&mut Inventory, With<Player>>,
) {
    let Ok(mut uses) = bread_uses.get_single_mut() else {
        return;
    };

    for player_entity in uses.read() {
        let mut inventory = player_query.get_mut(player_entity).unwrap();
        let held_item = inventory.held_item_stack_mut();

        held_item.take(1);
    }
}

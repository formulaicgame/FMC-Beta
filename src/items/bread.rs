use fmc::{items::Items, players::Player, prelude::*};

use crate::players::{HealEvent, Inventory};

use super::{ItemRegistry, ItemUses};

pub struct BreadPlugin;
impl Plugin for BreadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_bread)
            .add_systems(Update, eat_bread.after(super::ItemUseSystems));
    }
}

#[derive(Component)]
struct Bread;

fn register_bread(
    mut commands: Commands,
    items: Res<Items>,
    mut usable_items: ResMut<ItemRegistry>,
) {
    usable_items.insert(
        items.get_id("bread").unwrap(),
        commands.spawn((ItemUses::default(), Bread)).id(),
    );
}

fn eat_bread(
    mut bread_uses: Query<&mut ItemUses, (With<Bread>, Changed<ItemUses>)>,
    mut player_query: Query<&mut Inventory, With<Player>>,
    mut heal_events: EventWriter<HealEvent>,
) {
    let Ok(mut uses) = bread_uses.get_single_mut() else {
        return;
    };

    for player_entity in uses.read() {
        let mut inventory = player_query.get_mut(player_entity).unwrap();
        let held_item = inventory.held_item_stack_mut();

        heal_events.send(HealEvent {
            player_entity,
            healing: 8,
        });

        held_item.take(1);
    }
}

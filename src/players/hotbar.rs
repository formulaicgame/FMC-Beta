use fmc::{
    networking::{NetworkMessage, Server},
    players::Player,
    prelude::*,
    protocol::messages,
};

use crate::players::Hotbar;

pub struct HotbarPlugin;
impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (initialize_interface, send_server_updates, equip_item),
        );
    }
}

fn initialize_interface(net: Res<Server>, new_player_query: Query<Entity, Added<Player>>) {
    for player_entity in new_player_query.iter() {
        net.send_one(
            player_entity,
            messages::InterfaceVisibilityUpdate {
                interface_path: "hotbar".to_owned(),
                visible: true,
            },
        );
    }
}

fn send_server_updates(net: Res<Server>, hotbar_query: Query<(Entity, &Hotbar), Changed<Hotbar>>) {
    for (player_entity, hotbar) in hotbar_query.iter() {
        let mut hotbar_node = messages::InterfaceItemBoxUpdate::default();
        for (i, item_stack) in hotbar.iter().enumerate() {
            if let Some(item) = item_stack.item() {
                hotbar_node.add_itembox(
                    "hotbar",
                    i as u32,
                    item.id,
                    item_stack.size(),
                    item.properties["durability"].as_u64().map(|v| v as u32),
                    item.properties["description"].as_str(),
                );
            } else {
                hotbar_node.add_empty_itembox("hotbar", i as u32);
            }
        }

        net.send_one(player_entity, hotbar_node);
    }
}
fn equip_item(
    net: Res<Server>,
    mut equip_events: EventReader<NetworkMessage<messages::InterfaceEquipItem>>,
    mut hotbar: Query<&mut Hotbar>,
) {
    for equip_event in equip_events.read() {
        if equip_event.interface_path != "hotbar" {
            return;
        }

        if equip_event.index > 8 {
            net.disconnect(equip_event.player_entity);
            continue;
        }

        let mut hotbar = hotbar.get_mut(equip_event.player_entity).unwrap();
        hotbar.equipped_item = equip_event.index as usize;
    }
}

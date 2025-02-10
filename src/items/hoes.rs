use fmc::{
    bevy::math::DVec3,
    blocks::{BlockId, Blocks},
    items::{Item, ItemStack, Items},
    networking::Server,
    players::{Player, Target, Targets},
    prelude::*,
    protocol::messages,
    utils::Rng,
    world::{chunk::ChunkPosition, BlockUpdate, ChunkSubscriptions},
};

use super::{DroppedItem, ItemRegistry, ItemUses};

pub struct HoePlugin;
impl Plugin for HoePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_hoes)
            .add_systems(Update, use_hoe.after(super::ItemUseSystems));
    }
}

fn register_hoes(
    mut commands: Commands,
    blocks: Res<Blocks>,
    items: Res<Items>,
    mut usable_items: ResMut<ItemRegistry>,
) {
    usable_items.insert(
        items.get_id("hoe").unwrap(),
        commands
            .spawn((
                ItemUses::default(),
                HoeConfig {
                    dirt: blocks.get_id("dirt"),
                    grass: blocks.get_id("grass"),
                },
            ))
            .id(),
    );
}

#[derive(Component)]
struct HoeConfig {
    pub dirt: BlockId,
    pub grass: BlockId,
}

fn use_hoe(
    mut commands: Commands,
    net: Res<Server>,
    items: Res<Items>,
    chunk_subscriptions: Res<ChunkSubscriptions>,
    player_query: Query<&Targets, With<Player>>,
    mut hoe_uses: Query<(&mut ItemUses, &HoeConfig), Changed<ItemUses>>,
    mut block_update_writer: EventWriter<BlockUpdate>,
    mut rng: Local<Rng>,
) {
    let Ok((mut uses, config)) = hoe_uses.get_single_mut() else {
        return;
    };

    for player_entity in uses.read() {
        let targets = player_query.get(player_entity).unwrap();

        let Some(Target::Block {
            block_position,
            block_id,
            ..
        }) = targets
            .get_first_block(|block_id| *block_id == config.dirt || *block_id == config.grass)
        else {
            continue;
        };

        if *block_id == config.grass {
            let item_config = items.get_config_by_name("wheat_seeds").unwrap();
            let item_stack = ItemStack::new(item_config, 1);
            commands.spawn((
                DroppedItem::new(item_stack),
                Transform::from_translation(block_position.as_dvec3() + DVec3::new(0.5, 1.1, 0.5)),
            ));
        }

        let blocks = Blocks::get();
        let soil_id = blocks.get_id("soil");
        let block_config = blocks.get_config(&soil_id);

        let chunk_position = ChunkPosition::from(*block_position);
        if let Some(subscribers) = chunk_subscriptions.get_subscribers(&chunk_position) {
            let position = block_position.as_dvec3() + DVec3::splat(0.5);

            if let Some(place_sound) = block_config.sound.place(&mut rng) {
                net.send_many(
                    subscribers,
                    messages::Sound {
                        position: Some(position),
                        volume: 1.0,
                        speed: 1.0,
                        sound: place_sound.to_owned(),
                    },
                )
            }
        }

        block_update_writer.send(BlockUpdate::Change {
            position: *block_position,
            block_id: soil_id,
            block_state: None,
        });
    }
}

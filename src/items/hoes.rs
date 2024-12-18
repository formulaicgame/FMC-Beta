use fmc::{
    bevy::math::DVec3,
    blocks::{BlockId, Blocks},
    items::Items,
    models::Models,
    players::{Player, Target, Targets},
    prelude::*,
    world::{BlockUpdate, WorldMap},
};

use super::{GroundItemBundle, ItemUses, UsableItems};

pub struct HoePlugin;
impl Plugin for HoePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_hoes)
            .add_systems(Update, use_hoe.after(super::RegisterItemUse));
    }
}

fn register_hoes(
    mut commands: Commands,
    blocks: Res<Blocks>,
    items: Res<Items>,
    mut usable_items: ResMut<UsableItems>,
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
    items: Res<Items>,
    models: Res<Models>,
    player_query: Query<&Targets, With<Player>>,
    mut hoe_uses: Query<(&mut ItemUses, &HoeConfig), Changed<ItemUses>>,
    mut block_update_writer: EventWriter<BlockUpdate>,
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
            let item_id = items.get_id("wheat_seeds").unwrap();
            let item_config = items.get_config(&item_id);

            commands.spawn(GroundItemBundle::new(
                item_id,
                item_config,
                models.get_by_id(item_config.model_id),
                1,
                block_position.as_dvec3() + DVec3::new(0.5, 1.1, 0.5),
            ));
        }

        block_update_writer.send(BlockUpdate::Change {
            position: *block_position,
            block_id: Blocks::get().get_id("soil"),
            block_state: None,
        });
    }
}

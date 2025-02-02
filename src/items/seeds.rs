use fmc::{
    blocks::{BlockId, Blocks},
    items::Items,
    players::{Camera, Player, Target, Targets},
    prelude::*,
    world::{BlockUpdate, WorldMap},
};

use super::{ItemRegistry, ItemUses};

pub struct SeedPlugin;
impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_seeds)
            .add_systems(Update, use_seeds.after(super::ItemUseSystems));
    }
}

fn register_seeds(
    mut commands: Commands,
    blocks: Res<Blocks>,
    items: Res<Items>,
    mut usable_items: ResMut<ItemRegistry>,
) {
    usable_items.insert(
        items.get_id("wheat_seeds").unwrap(),
        commands
            .spawn((
                ItemUses::default(),
                SeedConfig {
                    air: blocks.get_id("air"),
                    soil: blocks.get_id("soil"),
                },
            ))
            .id(),
    );
}

#[derive(Component)]
struct SeedConfig {
    pub air: BlockId,
    pub soil: BlockId,
}

fn use_seeds(
    world_map: Res<WorldMap>,
    player_query: Query<&Targets, With<Player>>,
    mut hoe_uses: Query<(&mut ItemUses, &SeedConfig), Changed<ItemUses>>,
    mut block_update_writer: EventWriter<BlockUpdate>,
) {
    let Ok((mut uses, config)) = hoe_uses.get_single_mut() else {
        return;
    };

    for player_entity in uses.read() {
        let targets = player_query.get(player_entity).unwrap();

        let Some(Target::Block { block_position, .. }) =
            targets.get_first_block(|block_id| *block_id == config.soil)
        else {
            continue;
        };

        if let Some(above_block) = world_map.get_block(*block_position + IVec3::Y) {
            if above_block != config.air {
                continue;
            }
        } else {
            continue;
        }

        block_update_writer.send(BlockUpdate::Change {
            position: *block_position + IVec3::Y,
            block_id: Blocks::get().get_id("wheat_0"),
            block_state: None,
        });
    }
}

use fmc::{
    bevy::math::DVec3,
    blocks::{BlockRotation, Blocks},
    items::{Item, ItemStack, Items},
    prelude::*,
    world::{BlockUpdate, ChangedBlockEvent},
};

use crate::items::DroppedItem;

pub struct TorchPlugin;
impl Plugin for TorchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fragile_break);
    }
}

// TODO: Fragility will apply to many blocks. Make a 'fragile' flag in the block config and make a
// general system.
//
// Make the torch break if the block it is connected to is removed.
fn fragile_break(
    mut commands: Commands,
    items: Res<Items>,
    mut changed_blocks: EventReader<ChangedBlockEvent>,
    mut block_updates: EventWriter<BlockUpdate>,
) {
    for changed_block in changed_blocks.read() {
        for (block, block_rotation) in [
            (changed_block.right, Some(BlockRotation::Right)),
            (changed_block.left, Some(BlockRotation::Left)),
            (changed_block.front, Some(BlockRotation::Front)),
            (changed_block.back, Some(BlockRotation::Back)),
            (changed_block.top, None),
        ] {
            let Some(block) = block else {
                continue;
            };

            let torch_id = Blocks::get().get_id("torch");
            if block.0 != torch_id {
                continue;
            }

            if let Some(block_state) = block.1 {
                if block_rotation == block_state.rotation() {
                    let position = changed_block.position
                        + match block_rotation {
                            Some(BlockRotation::Front) => IVec3::Z,
                            Some(BlockRotation::Right) => IVec3::X,
                            Some(BlockRotation::Back) => IVec3::NEG_Z,
                            Some(BlockRotation::Left) => IVec3::NEG_X,
                            None => IVec3::Y,
                        };
                    block_updates.send(BlockUpdate::Change {
                        position,
                        block_id: Blocks::get().get_id("air"),
                        block_state: None,
                    });

                    let block_config = Blocks::get().get_config(&torch_id);
                    let (dropped_item_id, count) = match block_config.drop(None) {
                        Some(drop) => drop,
                        None => continue,
                    };

                    let item_config = items.get_config(&dropped_item_id);
                    let item_stack = ItemStack::new(item_config, count);

                    commands.spawn((
                        DroppedItem::new(item_stack),
                        Transform::from_translation(position.as_dvec3() + DVec3::splat(0.5)),
                    ));
                }
            }
        }
    }
}

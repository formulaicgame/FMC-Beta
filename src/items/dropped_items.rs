use fmc::{
    bevy::math::DVec3,
    items::{ItemStack, Items},
    models::{Model, ModelMap, Models},
    physics::{Collider, Physics},
    prelude::*,
    utils::Rng,
    world::chunk::ChunkPosition,
};

use crate::players::Hotbar;

pub struct DroppedItemsPlugin;
impl Plugin for DroppedItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pick_up_items)
            .add_systems(Update, manage_item_models.in_set(DropItems));
    }
}

/// Order systems that drop blocks before this systemset to avoid 1-frame lag.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DropItems;

// An item that is dropped on the ground.
#[derive(Component, Deref, DerefMut)]
#[require(Transform)]
pub struct DroppedItem(ItemStack);

impl DroppedItem {
    pub fn new(item_stack: ItemStack) -> Self {
        Self(item_stack)
    }
}

fn manage_item_models(
    mut commands: Commands,
    models: Res<Models>,
    items: Res<Items>,
    mut dropped_items: Query<
        (Entity, &DroppedItem, Option<&Physics>, &mut Transform),
        Added<DroppedItem>,
    >,
    mut rng: Local<Rng>,
) {
    for (entity, dropped_item, maybe_physics, mut transform) in dropped_items.iter_mut() {
        let item_id = dropped_item.0.item().unwrap().id;
        let item_config = items.get_config(&item_id);
        let model_config = models.get_by_id(item_config.model_id);

        // TODO: This won't work if the model must be scaled up
        //
        // We want dropped items to have a uniform size. If the model's width is larger than
        // HALF_SIZE*2 we scale it by width. If it is already smaller than that, we instead scale
        // the height.
        const HALF_SIZE: f64 = 0.1;
        let mut aabb = model_config.aabb.clone();
        let xz_scale = HALF_SIZE / aabb.half_extents.x.max(aabb.half_extents.z);
        let y_scale = HALF_SIZE * 1.5 / aabb.half_extents.y;
        let scale = if xz_scale < 1.0 { xz_scale } else { y_scale };

        transform.scale = DVec3::splat(scale);
        // Moving the center down will make the model float.
        aabb.center.y -= 0.1;

        let mut entity_commands = commands.entity(entity);

        entity_commands.insert((Model::Asset(item_config.model_id), Collider::Aabb(aabb)));

        if maybe_physics.is_none() {
            let random = rng.next_f32() * std::f32::consts::TAU;
            let velocity_x = random.sin() as f64 * 3.0;
            let velocity_z = random.cos() as f64 * 3.0;
            let velocity_y = 6.5;

            entity_commands.insert(Physics {
                enabled: true,
                velocity: DVec3::new(velocity_x, velocity_y, velocity_z),
                ..default()
            });
        }
    }
}

fn pick_up_items(
    mut commands: Commands,
    model_map: Res<ModelMap>,
    mut players: Query<(&GlobalTransform, &mut Hotbar), Changed<GlobalTransform>>,
    mut dropped_items: Query<(Entity, &mut DroppedItem, &Transform)>,
) {
    for (player_position, mut player_hotbar) in players.iter_mut() {
        let chunk_position = ChunkPosition::from(player_position.translation());
        let item_entities = match model_map.get_entities(&chunk_position) {
            Some(e) => e,
            None => continue,
        };

        for item_entity in item_entities.iter() {
            if let Ok((entity, mut dropped_item, transform)) = dropped_items.get_mut(*item_entity) {
                if transform
                    .translation
                    .distance_squared(player_position.translation())
                    < 2.0
                {
                    // First test that the item can be picked up. This is to avoid triggering
                    // change detection for the hotbar. If detection is triggered, it will send
                    // an interface update to the client. Can't pick up = spam
                    let mut capacity = false;
                    for item_stack in player_hotbar.iter() {
                        if (item_stack.item() == dropped_item.item()
                            && item_stack.remaining_capacity() != 0)
                            || item_stack.is_empty()
                        {
                            capacity = true;
                            break;
                        }
                    }
                    if !capacity {
                        break;
                    }

                    // First try to fill item stacks that already have the item
                    for item_stack in player_hotbar.iter_mut() {
                        if item_stack.item() == dropped_item.item() {
                            dropped_item.transfer_to(item_stack, u32::MAX);
                        }

                        if dropped_item.is_empty() {
                            break;
                        }
                    }

                    if dropped_item.is_empty() {
                        commands.entity(entity).despawn();
                        continue;
                    }

                    // Then go again and fill empty spots
                    for item_stack in player_hotbar.iter_mut() {
                        if item_stack.is_empty() {
                            dropped_item.transfer_to(item_stack, u32::MAX);
                        }

                        if dropped_item.is_empty() {
                            break;
                        }
                    }

                    if dropped_item.is_empty() {
                        commands.entity(entity).despawn();
                        continue;
                    }
                }
            }
        }
    }
}

use fmc::prelude::*;

mod crafting_table;
mod torch;
mod water;
mod wheat;

pub(super) struct BlocksPlugin;
impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crafting_table::CraftingTablePlugin)
            .add_plugins(wheat::WheatPlugin)
            .add_plugins(torch::TorchPlugin)
            .add_plugins(water::WaterPlugin);
    }
}

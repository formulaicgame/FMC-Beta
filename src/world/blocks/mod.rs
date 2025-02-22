use fmc::prelude::*;

mod water;

pub(super) struct BlocksPlugin;
impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(water::WaterPlugin);
    }
}

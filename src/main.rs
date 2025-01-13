use fmc::bevy::{
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use fmc_beta::*;

mod assets;

fn main() {
    App::new()
        .insert_resource(settings::Settings::load())
        .add_plugins(assets::ExtractBundledAssetsPlugin)
        .add_plugins(fmc::DefaultPlugins)
        //.add_plugins((FrameTimeDiagnosticsPlugin, FrameCountPlugin))
        .add_plugins(items::ItemPlugin)
        .add_plugins(players::PlayerPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(skybox::SkyPlugin)
        .add_plugins(mobs::MobsPlugin)
        .run();
}

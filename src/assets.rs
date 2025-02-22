use fmc::prelude::*;

pub struct ExtractBundledAssetsPlugin;
impl Plugin for ExtractBundledAssetsPlugin {
    fn build(&self, _app: &mut App) {
        // Assets from both the game(this crate) + mods are included at compile time
        // through the build script.
        // 1. The assets are always available without having to fetch them from the web.
        // 2. We do not need to have a list of necessary blocks/items/models included in the
        //    source. Although if compiled without the required assets, it will cause runtime
        //    errors.
        let assets = include_bytes!(concat!(env!("OUT_DIR"), "/assets.tar.zstd"));
        let uncompressed = zstd::stream::decode_all(assets.as_slice()).unwrap();
        let mut archive = tar::Archive::new(uncompressed.as_slice());

        // TODO: Want to store assets the same place they are unpacked so we don't have to
        // contantly remove + write. This works fine for base game, but not for mods, as it would
        // unpack all the server assets into the mod's assets directory.
        //
        // Always overwrite when developing so assets are replaced when they are edited.
        if std::env::var_os("CARGO").is_some() {
            std::fs::remove_dir_all("assets").ok();
        };

        for entry in archive.entries().unwrap() {
            let mut file = entry.unwrap();
            let path = file.path().unwrap();
            if !path.exists() {
                match file.unpack_in(".") {
                    Err(e) => panic!("Failed to extract default assets.\nError: {e}"),
                    _ => (),
                }
            }
        }
    }
}

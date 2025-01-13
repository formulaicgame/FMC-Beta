use fmc::prelude::*;

pub struct ExtractBundledAssetsPlugin;
impl Plugin for ExtractBundledAssetsPlugin {
    fn build(&self, _app: &mut App) {
        // Assets from both the base server(this crate) and any mods are included at compile time
        // through the build script.
        // 1. The assets are always available without having to fetch them from the web.
        // 2. We do not need to have a list of necessary blocks/items/models included in the
        //    source. Although if compiled without the required assets, it will cause unexpected panics.
        let assets = include_bytes!(concat!(env!("OUT_DIR"), "/assets.tar.zstd"));
        let uncompressed = zstd::stream::decode_all(assets.as_slice()).unwrap();
        let mut archive = tar::Archive::new(uncompressed.as_slice());

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

use std::{collections::HashMap, path::PathBuf};

use cargo_metadata::DependencyKind;

// Compress the assets from game + mods into an archive that is included in the executable.
fn main() {
    println!("cargo:rerun-if-changed=assets");

    let mut archive = tar::Builder::new(Vec::new());
    // Start by adding the base assets from this crate so they can be overridden by mods.
    // A guard is included later that prevents them being added again if this crate is used as a
    // dependency.
    archive
        .append_dir_all("assets/client", "assets/client")
        .unwrap();
    archive
        .append_dir_all("assets/server", "assets/server")
        .unwrap();

    let mut asset_paths = HashMap::new();

    for path in get_dependency_paths() {
        // TODO: If a non-mod dependency has a assets/{client, server} folder(somewhat likely) it
        // will be included even though it shouldn't be. Need some way to verify that a dependency
        // is a mod, maybe check that it has 'fmc' or this crate(idk if it should be possible to
        // create server agnostic mods) in its resolved dependency graph.
        for asset_path in walk_dir(path.join("assets/client")) {
            let relative_asset_path = asset_path.strip_prefix(&path).unwrap().to_path_buf();
            asset_paths.insert(relative_asset_path, asset_path);
        }

        for asset_path in walk_dir(&path.join("assets/server")) {
            let relative_asset_path = asset_path.strip_prefix(&path).unwrap().to_path_buf();
            asset_paths.insert(relative_asset_path, asset_path);
        }
    }

    for (relative_path, absolute_path) in asset_paths {
        archive
            .append_path_with_name(absolute_path, relative_path)
            .unwrap();
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let compressed: Vec<u8> =
        zstd::stream::encode_all(archive.into_inner().unwrap().as_slice(), 19).unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("assets.tar.zstd");

    std::fs::write(dest_path, compressed).unwrap();
}

fn walk_dir<P: AsRef<std::path::Path>>(dir: P) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    let Ok(directory) = std::fs::read_dir(&dir) else {
        return files;
    };

    for entry in directory {
        let file_path = entry.unwrap().path();

        if file_path.is_dir() {
            let sub_files = walk_dir(&file_path);
            files.extend(sub_files);
        } else {
            files.push(file_path);
        }
    }

    files
}

fn get_dependency_paths() -> Vec<PathBuf> {
    let manifest_path = PathBuf::from(std::env::var_os("OUT_DIR").unwrap())
        // TODO: This is the best way I could find of locating the folder of the binary that is
        // being built.
        .join("../../../../..")
        .join("Cargo.toml");

    let meta = cargo_metadata::MetadataCommand::new()
        .cargo_path(std::env::var_os("CARGO").unwrap())
        .manifest_path(&manifest_path)
        .exec()
        .unwrap();

    // Linear search over 'packages' with string eq for each dependency would get expensive with many mods.
    // Pre-compute a hashmap for faster index lookup.
    let mut index_lookup = HashMap::with_capacity(meta.packages.len());
    for (index, package) in meta.packages.iter().enumerate() {
        index_lookup.insert(package.name.clone(), index);
    }

    // A bit of mangling necessary here because the dependencies returned in metadata are in
    // alphabetical order. The dependency order in Cargo.toml is the asset priority order, so we
    // have to build it independently. The 'toml' crate has a 'preserve order' feature we use.
    let mut mod_paths_unsorted = HashMap::new();

    let root_package = meta.root_package().unwrap();
    for dependency in root_package.dependencies.iter() {
        if dependency.kind != DependencyKind::Normal {
            continue;
        }

        let package = &meta.packages[index_lookup[&dependency.name]];

        if package.manifest_path == manifest_path {
            // The server library is being used as a dependency, skip it.
            continue;
        }

        mod_paths_unsorted.insert(
            package.name.clone(),
            PathBuf::from(package.manifest_path.parent().unwrap()),
        );
    }

    let mut mod_paths = Vec::new();

    let manifest =
        toml::from_str::<toml::Table>(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
    for (name, data) in manifest
        .get("dependencies")
        .and_then(|t| t.as_table())
        .into_iter()
        .flatten()
    {
        // Dependencies can be renamed by specifying the crate name in the 'package' field
        let name = data.get("package").and_then(|p| p.as_str()).unwrap_or(name);

        mod_paths.push(mod_paths_unsorted.remove(name).unwrap());
    }

    return mod_paths;
}

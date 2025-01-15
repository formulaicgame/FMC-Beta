use fmc_beta::prelude::*;

fn main() {
    // While developing we want all artifacts to go to a separate directory.
    if std::env::var_os("CARGO").is_some() {
        std::fs::create_dir("server").ok();
        std::env::set_current_dir("server").unwrap();
    }

    App::new().add_plugins(fmc_beta::DefaultPlugins).run();
}

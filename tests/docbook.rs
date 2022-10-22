use std::{fs, path::Path};

use ctor::ctor;

use log::info;
use pathdiff::diff_paths;
use pretty_env_logger::env_logger::{Builder, Env};

use walkdir::WalkDir;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

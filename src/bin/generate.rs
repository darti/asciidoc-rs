use std::fs::File;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};

use asciidoc_rs::relaxng::generate;

fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::new().default_filter_or("info")).init();

    let file = "./schemas/docbook.rng.xml";
    info!("parsing {}", file);

    let _g = generate(File::open(file)?)?;

    Ok(())
}

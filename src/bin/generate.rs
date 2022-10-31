use log::info;
use pretty_env_logger::env_logger::{Builder, Env};
use std::fs;

use asciidoc_rs::relaxng::generate;

fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::new().default_filter_or("info")).init();

    let input = "./tests/schemas/relaxng_3_1.rng.xml";
    info!("parsing {}", input);

    let src = fs::read_to_string(input)?;
    let doc = roxmltree::Document::parse(&src)?;

    generate(&doc)?;

    Ok(())
}

mod common;

use asciidoc_rs::relaxng::Grammar;
use log::info;
use quick_xml::de;

#[test]

fn test_3_1() -> anyhow::Result<()> {
    let xml = include_str!("./schemas/relaxng_3_1.rng.xml");

    let output: Grammar = de::from_str(&xml)?;

    Ok(())
}

// #[test]

fn test_test_suite() -> anyhow::Result<()> {
    let xml = include_str!("./schemas/test_suite.rng.xml");

    let output: Grammar = de::from_str(&xml)?;

    info!("Parsed\n{:?}", output);

    Ok(())
}

#[test]

fn test_incremental() -> anyhow::Result<()> {
    let xml = include_str!("./schemas/incremental.rng.xml");

    let output: Grammar = de::from_str(&xml)?;

    Ok(())
}

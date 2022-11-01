use ctor::ctor;
use indoc::indoc;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};

use super::*;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

#[test]
fn test_enum() -> anyhow::Result<()> {
    let src = Grammar {
        start: Start {
            pattern: Pattern::Ref {
                name: "toto".into(),
            },
        },
    };

    let xml = se::to_string(&src)?;
    info!("xml: {}", xml);
    let output: Grammar = de::from_str(&xml)?;
    info!("xml: {:?}", output);

    assert_eq!(src, output);

    Ok(())
}

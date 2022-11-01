use ctor::ctor;

use log::info;
use pretty_env_logger::env_logger::{Builder, Env};

use super::*;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

fn roundtrip(g: &Grammar) -> anyhow::Result<()> {
    let xml = se::to_string(&g)?;
    info!("xml: {}", xml);
    let output: Grammar = de::from_str(&xml)?;
    info!("struct: {:?}", output);

    assert_eq!(*g, output);

    Ok(())
}

#[test]
fn test_enum() -> anyhow::Result<()> {
    let g = Grammar {
        start: Start {
            pattern: Pattern::Ref {
                name: "toto".into(),
            },
        },
        define: vec![],
    };

    roundtrip(&g)
}

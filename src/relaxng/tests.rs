use std::fmt::Debug;

use anyhow::Ok;
use ctor::ctor;

use log::info;
use pretty_assertions::{assert_eq, assert_ne};
use pretty_env_logger::env_logger::{Builder, Env};
use quick_xml::{de::from_str, se::Serializer, Writer};
use serde::Serialize;

use super::*;

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

fn roundtrip<T>(g: &T) -> anyhow::Result<()>
where
    T: Serialize + Debug + PartialEq + for<'de> Deserialize<'de>,
{
    let mut buffer = Vec::new();
    let writer = Writer::new_with_indent(&mut buffer, b' ', 2);

    let mut serializer = Serializer::with_root(writer, None);
    g.serialize(&mut serializer).unwrap();
    let xml = String::from_utf8(buffer).unwrap();

    let output: T = from_str(&xml)?;

    assert_eq!(*g, output);

    Ok(())
}

#[test]
fn test_enum() -> anyhow::Result<()> {
    let g = GrammarBuilder::default()
        .start(Start::new(reference("toto")))
        .build()?;

    roundtrip(&g)
}

#[test]
fn test_define() -> anyhow::Result<()> {
    let g = GrammarBuilder::default()
        .define(Define::new(
            "testSuite",
            vec![element(
                "testSuite",
                vec![
                    reference("header"),
                    zero_or_more(vec![choice(vec![
                        reference("testSuite"),
                        reference("testCase"),
                    ])]),
                ],
            )],
        ))
        .build()?;

    roundtrip(&g)
}

#[test]
fn test_define_choice() -> anyhow::Result<()> {
    let g = GrammarBuilder::default()
        .define(Define::new(
            "testCase",
            vec![element(
                "testCase",
                vec![
                    reference("header"),
                    reference("resouces"),
                    choice(vec![element(
                        "incorrect",
                        vec![reference("dtd"), reference("anyElement")],
                    )]),
                ],
            )],
        ))
        .build()?;

    // info!("grammar\n{}", quick_xml::se::to_string(&g)?);

    roundtrip(&g)
}

#[test]
fn test_name_class_name() -> anyhow::Result<()> {
    let src = NameClass::Name("toto".into());

    info!("xml\n{}", quick_xml::se::to_string(&src)?);

    roundtrip(&src)
}

#[test]
fn test_name_class_any_name_empty() -> anyhow::Result<()> {
    let src = NameClass::AnyName { except: None };

    roundtrip(&src)
}

#[test]
fn test_name_class_any_name_some() -> anyhow::Result<()> {
    let src = NameClass::AnyName {
        except: Some(Box::new(NameClass::Except(vec![NameClass::Name(
            "toto".into(),
        )]))),
    };

    roundtrip(&src)
}

#[test]
fn test_attribute_anyname_empty() -> anyhow::Result<()> {
    let src = attribute_with_name_class(NameClass::AnyName { except: None }, empty());

    info!("xml\n{}", quick_xml::se::to_string(&src)?);

    roundtrip(&src)
}

#[test]
fn test_attribute_anyname_value() -> anyhow::Result<()> {
    let src = attribute_with_name_class(
        NameClass::AnyName {
            except: Some(Box::new(NameClass::Except(vec![NameClass::Name(
                "toto".into(),
            )]))),
        },
        empty(),
    );

    info!("xml\n{}", quick_xml::se::to_string(&src)?);

    roundtrip(&src)
}

use std::{fmt::Debug, io::Cursor};

use anyhow::Ok;
use ctor::ctor;

use log::info;
use pretty_assertions::{assert_eq, assert_ne};
use pretty_env_logger::env_logger::{Builder, Env};
use quick_xml::{
    de::from_str,
    se::{to_string, to_writer},
    Writer,
};

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
    let xml = to_string(g)?;

    let output: T = from_str(&xml)?;

    assert_eq!(*g, output);

    Ok(())
}

fn deserialze_test<T>(xml: &str, g: &T) -> anyhow::Result<()>
where
    T: Serialize + Debug + PartialEq + for<'de> Deserialize<'de>,
{
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

// #[test]
// fn test_name_class_name() -> anyhow::Result<()> {
//     let src = NameClass::Name("toto".into());

//     roundtrip(&src)
// }

// #[test]
// fn test_name_class_any_name_empty() -> anyhow::Result<()> {
//     let src = NameClass::AnyName { except: None };

//     roundtrip(&src)
// }

#[test]
fn test_attribute_anyname_empty() -> anyhow::Result<()> {
    let src = attribute_with_name_class(NameClass::AnyName { except: None }, None);

    roundtrip(&src)
}

// #[test]
// fn test_attribute_anyname_value() -> anyhow::Result<()> {
//     let src = attribute_with_name_class(
//         NameClass::AnyName {
//             except: Box::new(NameClass::Name("toto".into())),
//         },
//         None,
//     );

//     roundtrip(&src)
// }

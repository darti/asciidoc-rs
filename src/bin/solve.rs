use log::info;
use pretty_env_logger::env_logger::{Builder, Env};

use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum AorB {
    A,
    B,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum CorD {
    C,
    D,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AorBonly {
    #[serde(rename = "$value")]
    a_b: AorB,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AorBandCorD {
    #[serde(rename = "$value")]
    a_b: AorB,

    #[serde(rename = "$value")]
    c_d: CorD,
}

fn roundtrip<T>(g: &T) -> anyhow::Result<()>
where
    T: Serialize + Debug + PartialEq + for<'de> Deserialize<'de>,
{
    info!("Input\n\n{:?}\n", g);

    let xml = to_string(g)?;
    info!("XML\n\n{}\n", xml);

    let output: T = from_str(&xml)?;
    info!("Output\n\n{:?}\n", output);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::new().default_filter_or("info")).init();

    let src = AorBandCorD {
        a_b: AorB::A,
        c_d: CorD::C,
    };

    roundtrip(&src)
}

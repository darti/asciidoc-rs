pub mod error;

use log::info;
use relaxng_syntax::{
    compact::{schema, Span},
    types::Decl,
    xml::parse,
};

use self::error::RelaxNgResult;

// pub fn generate_decl()

pub fn generate(s: &str) -> RelaxNgResult<()> {
    let schema = parse(s);

    info!("Parsed: {:?}", schema);

    // for decl in schema.decls {
    //     match decl {
    //         Decl::Datatypes(d) => info!("{:?}", d),
    //         _ => (),
    //     }
    // }

    Ok(())
}

mod declarations;
mod grammar;
mod identifiers;
mod names;
mod namespaces;

use ctor::ctor;

use pretty_env_logger::env_logger::{Builder, Env};

#[ctor]
fn init_logger() {
    Builder::from_env(Env::new().default_filter_or("info"))
        .is_test(true)
        .init();
}

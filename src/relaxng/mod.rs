use codegen::Scope;
use convert_case::{Case, Casing};
use log::info;
use roxmltree::{Document, Node};

use self::error::{RelaxNgError, RelaxNgResult};

pub mod error;

#[cfg(test)]
mod tests;

pub fn generate(doc: &Document) -> RelaxNgResult<()> {
    let root = doc.root_element();

    if root.tag_name().name() != "grammar" {
        return Err(RelaxNgError::Unsupported);
    }

    parse_grammar(&root)
}

fn parse_pattern(pat: &Node) -> RelaxNgResult<()> {
    let tag = pat.tag_name().name();
    info!("parse pattern, tag = {}", tag);

    match tag {
        "grammar" => parse_grammar(&pat),
        "element" => parse_element(&pat),
        t => Ok(()),
    }
}

fn parse_grammar(grammar: &Node) -> RelaxNgResult<()> {
    for def in grammar
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "define")
    {
        let name = def
            .attribute("name")
            .ok_or(RelaxNgError::ElementWithNoName)?;

        info!("definition {}", name);
        parse_define(&def)?;
    }

    let start = grammar
        .children()
        .find(|n| n.tag_name().name() == "start")
        .ok_or(RelaxNgError::MissingStart)?;

    Ok(())
}

fn parse_define(define: &Node) -> RelaxNgResult<()> {
    for def in define.children().filter(|n| n.is_element()) {
        parse_pattern(&def)?;
    }

    Ok(())
}

fn parse_element(elem: &Node) -> RelaxNgResult<()> {
    let name = elem
        .children()
        .find(|n| n.tag_name().name() == "name")
        .and_then(|n| n.text())
        .ok_or(RelaxNgError::ElementWithNoName)?;

    info!("type definition : {}", name);

    let mut scope = Scope::new();

    let elt = scope.new_struct(&name.to_case(Case::UpperCamel));

    info!("generated\n {}", scope.to_string());

    Ok(())
}

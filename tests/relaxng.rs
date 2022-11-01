mod common;

use asciidoc_rs::relaxng::Grammar;
use indoc::indoc;
use log::info;
use quick_xml::de;

#[test]
fn test_3_1() -> anyhow::Result<()> {
    let xml = indoc! {r#"
        <?xml version="1.0"?>
        <grammar xmlns="http://relaxng.org/ns/structure/1.0">
            <start>
                <ref name="foo.element" />
            </start>

            <define name="foo.element">
                <element>
                    <name ns="">foo</name>
                    <group>
                        <ref name="bar1.element" />
                        <ref name="bar2.element" />
                    </group>
                </element>
            </define>

            <define name="bar1.element">
                <element>
                    <name ns="http://www.example.com/n1">bar1</name>
                    <empty />
                </element>
            </define>

            <define name="bar2.element">
                <element>
                    <name ns="http://www.example.com/n2">bar2</name>
                    <empty />
                </element>
            </define>
        </grammar>
    "#};

    let output: Grammar = de::from_str(&xml)?;

    info!("Parsed\n{:?}", output);

    Ok(())
}

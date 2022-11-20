pub mod errors;

use std::borrow::Cow;
use std::io::{BufReader, Read};

use log::{error, info};
use quick_xml::events::attributes::{Attr, Attributes};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use self::errors::RelaxNgResult;

fn attribute_by_name<'a>(attrs: &'a mut Attributes, name: &[u8]) -> Option<Cow<'a, str>> {
    attrs.find_map(|a| match a {
        Ok(attr) if attr.key.local_name().as_ref() == name => attr.unescape_value().ok(),
        _ => None,
    })
}

pub fn generate<R>(r: R) -> RelaxNgResult<()>
where
    R: Read,
{
    let buffered = BufReader::new(r);

    let mut reader = Reader::from_reader(buffered);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"define" => info!(
                    "definition : {:?}",
                    attribute_by_name(&mut e.attributes(), b"name")
                ),
                _ => (),
            },

            Ok(Event::Eof) => break,
            Ok(_) => (),
            Err(e) => error!(target: "parsing", "parsing error: {}", e),
        }
    }

    Ok(())
}

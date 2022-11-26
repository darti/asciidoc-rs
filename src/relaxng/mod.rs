pub mod errors;

use std::borrow::Cow;
use std::io::{BufReader, Read};

use colored::Colorize;
use log::info;

use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use self::errors::RelaxNgResult;

enum Pattern {
    Element,
    Attribute,
}
enum State {
    Definition(String),
}

pub fn generate<R>(r: R) -> RelaxNgResult<()>
where
    R: Read,
{
    let buffered = BufReader::new(r);

    let mut reader = Reader::from_reader(buffered);
    reader.trim_text(true);

    let mut generator = Generator {
        buf: Vec::new(),
        state: Vec::new(),
        reader,
    };

    generator.init_state();

    Ok(())
}

struct Generator<R>
where
    R: Read,
{
    buf: Vec<u8>,
    state: Vec<State>,
    reader: Reader<BufReader<R>>,
}

impl<R> Generator<R>
where
    R: Read,
{
    fn init_state(&mut self) {
        loop {
            let evt = self.reader.read_event_into(&mut self.buf);

            match evt {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"define" => {
                        let mut attrs = e.attributes();
                        let name = attribute_by_name(&mut attrs, b"name");

                        if let Some(n) = name {
                            info!("{} {}", "Defining".blue(), n);
                            self.state.push(State::Definition(n.into()));
                        }
                    }
                    _ => (),
                },
                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"define" => {}
                    _ => (),
                },
                Ok(Event::Eof) => break,
                Err(e) => panic!(
                    "Error at position {}: {:?}",
                    self.reader.buffer_position(),
                    e
                ),
                Ok(_) => (),
            }

            self.buf.clear();
        }
    }
}

fn attribute_by_name<'a>(attrs: &'a mut Attributes, name: &[u8]) -> Option<Cow<'a, str>> {
    attrs.find_map(|a| match a {
        Ok(attr) if attr.key.local_name().as_ref() == name => attr.unescape_value().ok(),
        _ => None,
    })
}

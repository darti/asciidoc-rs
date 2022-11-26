pub mod errors;

use std::cell::RefCell;
use std::io::{BufReader, Read};
use std::rc::Rc;

use codegen::Scope;
use colored::Colorize;
use log::info;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

use self::errors::RelaxNgResult;

type Transition = Rc<dyn for<'a> Fn(Event<'a>) -> NextOp>;

enum NextOp {
    Push(Transition),
    Nop,
    Pop,
    Fail,
}

pub fn generate<R>(r: R) -> RelaxNgResult<()>
where
    R: Read,
{
    let buffered = BufReader::new(r);

    let mut reader = Reader::from_reader(buffered);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut funcs: Vec<Transition> = vec![Rc::new(init_state)];

    let reader = RefCell::new(reader);

    loop {
        let evt = reader.borrow_mut().read_event_into(&mut buf)?.clone();

        match evt {
            Event::Eof => break,
            e => {
                info!("event {:?}", e);

                if let Some(func) = funcs.last() {
                    let ne = e.clone();
                    match func(ne) {
                        NextOp::Push(f) => funcs.push(f),
                        NextOp::Nop => (),
                        NextOp::Pop => {
                            funcs.pop();
                        }
                        NextOp::Fail => panic!("processing error"),
                    }
                } else {
                    panic!("empty stack");
                }
            }
        }

        // self.buf.clear();
    }

    Ok(())
}

fn init_state<'a>(evt: Event<'a>) -> NextOp {
    match evt {
        Event::Start(e) if e.name().as_ref() == b"define" => {
            if let Some(n) = e
                .try_get_attribute(b"name")
                .ok()
                .flatten()
                .and_then(|a| a.unescape_value().ok())
            {
                let n = n.to_string();

                let c = define(n.clone());

                NextOp::Push(c)
            } else {
                NextOp::Nop
            }
        }
        _ => NextOp::Nop,
    }
}

fn define(name: String) -> Transition {
    info!("{} {}", "definition".blue().bold(), name);

    fn define_inner<'a>(evt: Event<'a>) -> NextOp {
        match evt {
            Event::Start(e) if e.name().as_ref() == b"element" => {
                // self.element(attribute_by_name(&mut attrs, b"name").to_string)
                NextOp::Nop
            }

            Event::End(e) if e.name().as_ref() == b"define" => NextOp::Pop,
            _ => NextOp::Fail,
        }
    }

    Rc::new(define_inner)
}

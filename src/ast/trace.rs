use std::{cell::Cell, iter::repeat};

static TRACE_IDENT_PLACEHOLDER: &'static str = "  ";

#[derive(Debug)]
pub struct Tracer {
    pub trace_level: usize,
    pub on: bool,
}

impl Tracer {
    pub fn new(on: bool) -> Tracer {
        Tracer {
            trace_level: 0,
            on: on,
        }
    }

    fn ident_level(&self) -> String {
        repeat(TRACE_IDENT_PLACEHOLDER)
            .take(self.trace_level)
            .collect::<String>()
    }

    fn inc(&mut self) {
        self.trace_level += 1;
    }

    fn dec(&mut self) {
        self.trace_level -= 1;
    }

    fn trace_print(&self, s: String) {
        if self.on {
            println!("{}{}", self.ident_level(), s)
        }
    }

    pub fn trace<'a>(&mut self, s: &'a str) -> Box<dyn Fn(&mut Tracer) -> &'a str + 'a> {
        self.trace_print(format!("BEGIN {}", String::from(s)));
        self.inc();
        Box::new(move |tr: &mut Tracer| tr.untrace(s))
    }

    pub fn untrace<'a>(&mut self, s: &'a str) -> &'a str {
        self.dec();
        self.trace_print(format!("END {}", s));
        s
    }
}

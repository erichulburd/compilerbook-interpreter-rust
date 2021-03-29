use std::{cell::Cell, iter::repeat};

static TRACE_IDENT_PLACEHOLDER: &'static str = "\t";


pub struct Tracer {
  trace_level: usize,
  pub on: Cell<bool>,
}

impl Tracer {
  pub fn new(on: bool) -> Tracer {
    Tracer{
      trace_level: 0,
      on: Cell::new(on),
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
    if self.on.get() {
      println!("{}{}", self.ident_level(), s)
    }
  }

  pub fn trace(&mut self, msg: String) -> String {
    self.inc();
    self.trace_print(format!("BEGIN {}", msg));
    msg
  }

  pub fn untrace(&mut self, msg: String) -> String {
    self.trace_print(format!("END {}", msg));
    self.dec();
    msg
  }
}




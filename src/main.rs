#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod ast;
mod lexer;
mod repl;
mod token;

fn main() {
    repl::start();
}

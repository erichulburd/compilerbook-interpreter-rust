#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod token;
mod lexer;
mod repl;
mod ast;

fn main() {
    repl::start();
}

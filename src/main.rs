#![allow(dead_code)]

#[macro_use(defer)]
extern crate scopeguard;

mod ast;
mod parser;
mod lexer;
mod repl;
mod token;

fn main() {
    repl::start();
}

#![allow(dead_code)]

#[macro_use(defer)]
extern crate scopeguard;

pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
mod token;

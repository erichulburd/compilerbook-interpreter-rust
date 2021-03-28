use super::expression::Expression;

pub type PrefixParseFn = Box<Fn() -> Expression>;
pub type InfixParseFn = Box<Fn(Expression) -> Expression>;

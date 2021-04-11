use super::{
    block_statement::BlockStatement, boolean_expression::BooleanExpression,
    call_expression::CallExpression, expression::Expression,
    expression_statement::ExpressionStatement, fn_literal::FnLiteral, identifier::Identifier,
    if_expression::IfExpression, infix_expression::InfixExpression,
    integer_literal::IntegerLiteral, let_statement::LetStatement,
    prefix_expression::PrefixExpression, program::Program, return_statement::ReturnStatement,
    statement::Statement,
};

pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
    BlockStatement(BlockStatement),
    BooleanExpression(BooleanExpression),
    CallExpression(CallExpression),
    ExpressionStatement(ExpressionStatement),
    FnLiteral(FnLiteral),
    Identifier(Identifier),
    IfExpression(IfExpression),
    InfixExpression(InfixExpression),
    IntegerLiteral(IntegerLiteral),
    LetStatement(LetStatement),
    PrefixExpression(PrefixExpression),
    ReturnStatement(ReturnStatement),
}

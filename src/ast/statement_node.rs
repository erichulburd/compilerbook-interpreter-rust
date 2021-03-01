use super::token_node::TokenNode;

pub trait StatementNode: TokenNode {
  fn statement_node(&self);
}

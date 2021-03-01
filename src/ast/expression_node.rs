use super::token_node::TokenNode;

pub trait ExpressionNode: TokenNode {
  fn expression_node(self);
}

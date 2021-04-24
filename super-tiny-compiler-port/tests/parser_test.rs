use super_tiny_compiler::models::{Token, TokenKind};
use super_tiny_compiler::parser::*;

#[test]
fn it_returns_ast() {
  assert_eq!(
    vec!(Token {
      kind: TokenKind::Paren,
      value: "(".to_owned()
    }),
    Ast {
      type: NodeType::Program,
      body: vec!(
      )
    }
  );
}

use super_tiny_compiler::models::{Ast, Node, Token, TokenKind};
use super_tiny_compiler::parser::*;

#[test]
fn it_returns_ast() {
  assert_eq!(
    vec!(Token {
      kind: TokenKind::Name,
      value: "(add 1 2)".to_owned()
    }),
    Ast {
      kind: Node::Program,
      body: vec!(Node::CallExpression {
        name: "add",
        params: vec!(
          Node::NumberLiteral { value: "1" },
          Node::NumberLiteral { value: "2" }
        )
      })
    }
  );
}

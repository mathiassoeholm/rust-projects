use super_tiny_compiler::models::{Ast, Node, Token};
use super_tiny_compiler::parser::*;

#[test]
fn it_returns_ast() {
  assert_eq!(
    parser(vec!(
      Token::Paren {
        value: "(".to_owned()
      },
      Token::Name {
        value: "add".to_owned()
      },
      Token::Number {
        value: "1".to_owned()
      },
      Token::Number {
        value: "2".to_owned()
      },
      Token::Paren {
        value: ")".to_owned()
      }
    )),
    Ast {
      body: vec!(Node::CallExpression {
        name: "add".to_owned(),
        params: vec!(
          Node::NumberLiteral {
            value: "1".to_owned()
          },
          Node::NumberLiteral {
            value: "2".to_owned()
          }
        )
      })
    }
  );
}

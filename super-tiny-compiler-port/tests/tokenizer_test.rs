use super_tiny_compiler::models::{Token, TokenKind};
use super_tiny_compiler::tokenizer::*;

#[test]
fn it_returns_tokens() {
  assert_eq!(
    tokenizer("(add 21 (subtract 4 2))"),
    Ok(vec!(
      Token {
        kind: TokenKind::Paren,
        value: String::from("(")
      },
      Token {
        kind: TokenKind::Name,
        value: String::from("add")
      },
      Token {
        kind: TokenKind::Number,
        value: String::from("21")
      },
      Token {
        kind: TokenKind::Paren,
        value: String::from("(")
      },
      Token {
        kind: TokenKind::Name,
        value: String::from("subtract")
      },
      Token {
        kind: TokenKind::Number,
        value: String::from("4")
      },
      Token {
        kind: TokenKind::Number,
        value: String::from("2")
      },
      Token {
        kind: TokenKind::Paren,
        value: String::from(")")
      },
      Token {
        kind: TokenKind::Paren,
        value: String::from(")")
      },
    ),)
  );
}

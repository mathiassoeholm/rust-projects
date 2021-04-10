use super_tiny_compiler::tokenizer;
use super_tiny_compiler::Token;
use super_tiny_compiler::TokenKind;

#[test]
fn it_returns_tokens() {
  assert_eq!(
    tokenizer("(add 2 (subtract 4 2))"),
    vec!(
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
        value: String::from("2")
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
    ),
  );
}

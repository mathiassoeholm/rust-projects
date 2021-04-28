use super_tiny_compiler::models::Token;
use super_tiny_compiler::tokenizer::*;

#[test]
fn it_returns_tokens() {
  assert_eq!(
    tokenizer("(add 21 (subtract 4 2))"),
    Ok(vec!(
      Token::Paren {
        value: String::from("(")
      },
      Token::Name {
        value: String::from("add")
      },
      Token::Number {
        value: String::from("21")
      },
      Token::Paren {
        value: String::from("(")
      },
      Token::Name {
        value: String::from("subtract")
      },
      Token::Number {
        value: String::from("4")
      },
      Token::Number {
        value: String::from("2")
      },
      Token::Paren {
        value: String::from(")")
      },
      Token::Paren {
        value: String::from(")")
      },
    ),)
  );
}

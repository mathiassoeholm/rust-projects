use super_tiny_compiler::models::Token;
use super_tiny_compiler::tokenizer::*;

#[test]
fn it_returns_tokens() {
  assert_eq!(
    tokenizer("(add 21 (subtract 4 2))"),
    Ok(vec!(
      Token::Paren { value: "(" },
      Token::Name { value: "add" },
      Token::Number { value: "21" },
      Token::Paren { value: "(" },
      Token::Name { value: "subtract" },
      Token::Number { value: "4" },
      Token::Number { value: "2" },
      Token::Paren { value: ")" },
      Token::Paren { value: ")" },
    ),)
  );
}

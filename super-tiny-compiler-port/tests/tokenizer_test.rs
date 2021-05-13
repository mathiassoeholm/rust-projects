use super_tiny_compiler::models::Token;
use super_tiny_compiler::tokenizer::*;

#[test]
fn it_returns_tokens() {
  let t: Vec<Token> = tokenizer("(add 21 (subtract 4 2))").unwrap().collect();
  println!("{:?}", t);
  assert_eq!(
    t,
    vec!(
      Token::Name { value: "(" },
      // Token::Name { value: "add" },
      // Token::Number { value: "21" },
      // Token::Paren { value: "(" },
      // Token::Name { value: "subtract" },
      // Token::Number { value: "4" },
      // Token::Number { value: "2" },
      // Token::Paren { value: ")" },
      // Token::Paren { value: ")" },
    )
  );
}

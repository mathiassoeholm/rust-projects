#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Paren,
  Name,
  Number,
}

#[derive(Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub value: String,
}

pub fn tokenizer(input: &str) -> Vec<Token> {
  let mut current = 0;
  let mut tokens = Vec::new();

  let chars: Vec<char> = input.chars().collect();
  while current < input.len() {
    let ch = chars[current];

    if ch == '(' {
      tokens.push(Token {
        kind: TokenKind::Paren,
        value: String::from("("),
      })
    }

    current += 1;
    continue;
  }

  return tokens;
}

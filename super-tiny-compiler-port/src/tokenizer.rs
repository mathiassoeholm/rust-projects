use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
  Paren,
  Name,
  Number,
  String,
}

#[derive(Debug, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub value: String,
}

pub fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
  let mut current = 0;
  let mut tokens = Vec::new();
  let white_space_regex = Regex::new(r"\s").unwrap();
  let number_regex = Regex::new(r"[0-9]").unwrap();
  let letters_regex = Regex::new(r"(?i)[a-z]").unwrap();

  let chars: Vec<char> = input.chars().collect();
  while current < input.len() {
    let mut ch = chars[current];

    if ch == '(' || ch == ')' {
      tokens.push(Token {
        kind: TokenKind::Paren,
        value: String::from(ch),
      });

      current += 1;
      continue;
    }

    if white_space_regex.is_match(&String::from(ch)) {
      current += 1;
      continue;
    }

    if number_regex.is_match(&String::from(ch)) {
      let mut value = Vec::new();

      while number_regex.is_match(&String::from(ch)) {
        value.push(ch);
        current += 1;
        ch = chars[current];
      }

      tokens.push(Token {
        kind: TokenKind::Number,
        value: value.iter().collect(),
      });

      continue;
    }

    if ch == '"' {
      let mut value = Vec::new();

      current += 1;
      ch = chars[current];

      while ch != '"' {
        value.push(ch);
        current += 1;
        ch = chars[current];
      }

      tokens.push(Token {
        kind: TokenKind::String,
        value: value.iter().collect(),
      });

      continue;
    }

    if letters_regex.is_match(&String::from(ch)) {
      let mut value = Vec::new();

      while letters_regex.is_match(&String::from(ch)) {
        value.push(ch);
        current += 1;
        ch = chars[current];
      }

      tokens.push(Token {
        kind: TokenKind::Name,
        value: value.iter().collect(),
      });

      continue;
    }

    return Err(format!("I don't know what this character is: {}", ch));
  }

  Ok(tokens)
}

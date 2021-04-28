use super::models::Token;
use regex::Regex;

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
      tokens.push(Token::Paren {
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

      tokens.push(Token::Number {
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

      tokens.push(Token::String {
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

      tokens.push(Token::Name {
        value: value.iter().collect(),
      });

      continue;
    }

    return Err(format!("I don't know what this character is: {}", ch));
  }

  Ok(tokens)
}

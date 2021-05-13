use super::models::Token;
use regex::Regex;

// static white_space_regex: Regex = Regex::new(r"\s").unwrap();
// static number_regex: Regex = Regex::new(r"[0-9]").unwrap();

pub fn tokenizer<'a>(input: &'a str) -> Result<impl Iterator<Item = Token<'a>>, String> {
  let letters_regex: Regex = Regex::new(r"(?i)[a-z]").unwrap();

  let mut letter_start = None;

  let ret = input.char_indices().filter_map(move |(i, c)| {
    let token = match c {
      ' ' => None,
      '(' => Some(Token::Paren {
        value: &input[i..(i + 1)],
      }),
      _ if letters_regex.is_match(&input[i..(i + 1)]) => {
        letter_start = Some(letter_start.unwrap_or(i));
        if i + 1 == input.len() || !letters_regex.is_match(&input[(i + 1)..(i + 2)]) {
          Some(Token::Name {
            value: &input[letter_start.unwrap()..(i + 1)],
          })
        } else {
          None
        }
      }
      _ => None,
    };

    token
  });

  Ok(ret)
  // let white_space_regex = Regex::new(r"\s").unwrap();
  // let number_regex = Regex::new(r"[0-9]").unwrap();
  // let letters_regex = Regex::new(r"(?i)[a-z]").unwrap();

  // let chars: Vec<char> = input.charks().collect();
  // while current < input.len() {
  //   let mut ch = chars[current];

  //   if ch == '(' || ch == ')' {
  //     tokens.push(Token::Paren {
  //       value: String::from(ch),
  //     });

  //     current += 1;
  //     continue;
  //   }

  //   if white_space_regex.is_match(&String::from(ch)) {
  //     current += 1;
  //     continue;
  //   }

  //   if number_regex.is_match(&String::from(ch)) {
  //     let mut value = Vec::new();

  //     while number_regex.is_match(&String::from(ch)) {
  //       value.push(ch);
  //       current += 1;
  //       ch = chars[current];
  //     }

  //     tokens.push(Token::Number {
  //       value: value.iter().collect(),
  //     });

  //     continue;
  //   }

  //   if ch == '"' {
  //     let mut value = Vec::new();

  //     current += 1;
  //     ch = chars[current];

  //     while ch != '"' {
  //       value.push(ch);
  //       current += 1;
  //       ch = chars[current];
  //     }

  //     tokens.push(Token::String {
  //       value: value.iter().collect(),
  //     });

  //     continue;
  //   }

  //   if letters_regex.is_match(&String::from(ch)) {
  //     let mut value = Vec::new();

  //     while letters_regex.is_match(&String::from(ch)) {
  //       value.push(ch);
  //       current += 1;
  //       ch = chars[current];
  //     }

  //     tokens.push(Token::Name {
  //       value: value.iter().collect(),
  //     });

  //     continue;
  //   }

  //   return Err(format!("I don't know what this character is: {}", ch));
  // }

  // Ok(tokens)
}

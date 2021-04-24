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

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

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
  CallExpression {
    name: &'a String,
    params: Vec<Node<'a>>,
  },
  NumberLiteral {
    value: &'a String,
  },
}

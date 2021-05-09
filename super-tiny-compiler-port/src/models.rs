#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Paren { value: &'a str },
  Name { value: &'a str },
  Number { value: &'a str },
  String { value: &'a str },
}

#[derive(Debug, PartialEq)]
pub enum Node {
  Program { body: Vec<Node> },
  CallExpression { name: String, params: Vec<Node> },
  NumberLiteral { value: String },
}

#[derive(Debug, PartialEq)]
pub struct Ast {
  body: Vec<Node>,
}

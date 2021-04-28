#[derive(Debug, PartialEq)]
pub enum Token {
  Paren { value: String },
  Name { value: String },
  Number { value: String },
  String { value: String },
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

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn add_text(&mut self, text: &str) {
    self.content = self
      .state
      .as_ref()
      .unwrap()
      .add_text(self.content.clone(), text);
  }

  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
  }

  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }

  pub fn reject(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.reject())
    }
  }
}

trait State {
  fn add_text(&self, current_text: String, text_to_add: &str) -> String;
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn reject(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview { approvals: 0 })
  }

  fn approve(self: Box<Self>) -> Box<(dyn State)> {
    self
  }

  fn reject(self: Box<Self>) -> Box<(dyn State)> {
    self
  }

  fn add_text(&self, current_text: String, text_to_add: &str) -> String {
    current_text + text_to_add
  }
}

struct PendingReview {
  approvals: u8,
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(mut self: Box<Self>) -> Box<(dyn State)> {
    self.approvals += 1;
    if self.approvals == 2 {
      Box::new(Published {})
    } else {
      self
    }
  }

  fn reject(self: Box<Self>) -> Box<(dyn State)> {
    Box::new(Draft {})
  }

  fn add_text(&self, current_text: String, _: &str) -> String {
    current_text
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<(dyn State)> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }

  fn reject(self: Box<Self>) -> Box<(dyn State)> {
    self
  }

  fn add_text(&self, current_text: String, _: &str) -> String {
    current_text
  }
}

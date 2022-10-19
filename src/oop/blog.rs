pub struct Post {
  content: String,
}
pub struct DraftPost {
  content: String,
}
impl Post {
  pub fn new() -> DraftPost {
      DraftPost { content: String::new() }
  }
  pub fn content(&self) -> &str {
      &self.content
  }
}
impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
      self.content.push_str(text);
  }
  pub fn request_review(self) -> PendingReviewPost {
      PendingReviewPost {
          content: self.content,
      }
  }
}

pub struct PendingReviewPost {
  content: String,
}
impl PendingReviewPost {
  pub fn approve(self) -> Post {
      Post { content: self.content }
  }
}





// pub struct Post {
//     pub state: Option<Box<dyn State>>,
//     content: String,
// }
// impl Post {
//     pub fn new() -> Self {
//         Self {
//             state: Some(Box::new(Draft {})),
//             content: String::new()
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }

//     pub fn reject(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.reject())
//         }
//     }
// }

// pub trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, _post: &'a Post) -> &'a str {
//         ""
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State>;
// }

// struct Draft {}
// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview { count: 1 })
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }


// struct PendingReview {
//     count: i32,
// }
// impl PendingReview {
//     fn count_add(&mut self) {
//         self.count += 1;
//     }
//     fn count(&self) -> i32 {
//         self.count
//     }
// }
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(mut self: Box<Self>) -> Box<dyn State> {
//         if self.count() == 1 {
//             Box::new(Published {})
//         } else {
//             self.count_add();
//             self
//         }
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }
// }

// struct Published {}
// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }

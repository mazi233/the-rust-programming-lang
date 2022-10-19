mod attrs;
mod blog;
mod gui;

use gui::{Screen, Draw, Button};
use blog::{Post};

pub fn main() {
  // attrs::main();
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();


  let mut post = Post::new();
  post.add_text("I ate a salad for lunch today");
  let post = post.request_review();
  let post = post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}
impl Draw for SelectBox {
  fn draw(&self) {
    
  }
}

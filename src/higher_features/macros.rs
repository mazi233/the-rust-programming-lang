use hello_macro::HelloMacro;
use hello_macro_derive::{
  HelloMacro,
  route,
  sql,
};

#[derive(HelloMacro)]
struct Pancakes;

// #[macro_export] 注解表明只要导入了定义这个宏的crate，该宏就应该是可用的
#[macro_export]
macro_rules! vec2 {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}

pub fn main() {
  assert_eq!(vec![1, 2, 3], vec2![1, 2, 3]);

  Pancakes::hello_macro();

  // let s = sql!(SELECT * FROM posts WHERE id=1);
}

#[route("GET", "/")]
fn index() {}

use std::{ops::Deref, rc::Rc};

use self::List::{Cons, Nil};
use self::RcList::{RcCons, RcNil};

pub fn main() {
  let b = Box::new(5);
  println!("b = {}", b);

  // let list = Cons(1, Cons(2, Cons(3, Nil)));
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("{:?}", list);

  let x = 5;
  let y = &x;
  assert_eq!(5, *y);
  let y = Box::new(x);
  assert_eq!(5, x);
  assert_eq!(5, *y);

  let x = 5;
  let y = MyBox::new(x);
  assert_eq!(5, *y);
  
  let m = MyBox::new(String::from("Rust"));
  hello(&m);

  let _c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let _d = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  let c = CustomSmartPointer {
    data: String::from("some data"),
  };
  println!("CustomSmartPointer created.");
  drop(c);
  println!("CustomSmartPointers created.");

  // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
  // let b = Cons(3, Box::new(a));
  // let c = Cons(4, Box::new(a));

  let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let _b = RcCons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let _c = RcCons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
  Cons(i32, Box<List>),
  Nil,
}
#[derive(Debug)]
enum RcList {
  RcCons(i32, Rc<RcList>),
  RcNil,
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
  fn new(x: T) -> Self {
    Self(x)
  }
}
impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

struct CustomSmartPointer {
  data: String
}
impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

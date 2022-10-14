use std::{ops::Deref, rc::Rc};
use std::cell::RefCell;


use self::List::{Cons, Nil};
use self::RcList::{RcCons, RcNil};
use self::RcRefCellList::{RcRefCellCons, RcRefCellNil};

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

  let value = Rc::new(RefCell::new(5));
  let a = Rc::new(RcRefCellCons(Rc::clone(&value), Rc::new(RcRefCellNil)));

  let b = RcRefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = RcRefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
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
#[derive(Debug)]
enum RcRefCellList {
  RcRefCellCons(Rc<RefCell<i32>>, Rc<RcRefCellList>),
  RcRefCellNil,
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

pub trait Messenger {
  fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }
  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;
    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self.messenger
        .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self.messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }
  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger { sent_messages: RefCell::new(vec![]) }
    }
  }
  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}

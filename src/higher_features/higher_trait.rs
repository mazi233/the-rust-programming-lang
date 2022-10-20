use std::ops::{Add, Deref};

pub fn main() {
  let mm1 = Millimeters(1);
  let mm2 = Meters(2);
  println!("{:?}", mm1 + mm2);

  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();

  println!("A baby dog is called a {}", Dog::baby_name());
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);

  println!("{:?}", *w.first().unwrap());
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Self::Output {
    Millimeters(self.0 + (other.0 * 1000))
  }
}

trait Pilot {
  fn fly(&self);
}
trait Wizard {
  fn fly(&self);
}
struct Human;
impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}
impl Wizard for Human {
  fn fly(&self) {
    println!("p!");
  }
}
impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

use std::fmt;
trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

struct Point {
  x: i32,
  y: i32,
}
impl OutlinePrint for Point {}
impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

impl Deref for Wrapper {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

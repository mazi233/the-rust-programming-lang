use std::fmt::{Display, Debug};

pub fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);

  let result = largest2(&number_list);
  println!("The largest2 number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest2(&char_list);
  println!("The largest2 char is {}", result);

  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c' };
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
      "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());

  notify(&tweet);

  let p = Pair::new(3, 2);
  p.cmp_display();

  HHH::hhh();

  let string1 = String::from("abcd");
  let string2 = "xyz";
  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);

  let string1 = String::from("long string is long");
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };

  println!("{}", longest_with_an_announcement("123", "42221", "asdsd"));
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {3}
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn longest_with_an_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T,
) -> &'a str
  where
    T: Display
{
  println!("ann: {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

trait Hhh {
  fn hhh();
}

trait Hhh2 {
  fn hhh2();
}

trait Hhh3 {
  fn hhh3();
}

impl<T: Hhh2> Hhh for T {
  fn hhh() {
    println!("hhh");
  }
}

struct HHH;
impl Hhh for HHH {
  fn hhh() {
    println!("hhhhhhhh");
  }
}

struct Pair<T> {
  x: T,
  y: T,
}
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn largest2<T: PartialOrd + Copy>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

struct Point<X1, Y1> {
  x: X1,
  y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
  fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
    Point { x: self.x, y: other.y }
  }
}

pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {
  println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

pub fn notify3(item: &(impl Summary + Display)) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

// fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_fn<T, U>(t: &T, u: &U) -> i32
  where T: Display + Clone,
        U: Clone + Debug
{
  1
}

fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

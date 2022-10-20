pub fn main() {
  let answer = do_twice(add_one, 5);
  println!("The answer is: {}", answer);

  println!("{}", returns_closure()(1));
  println!("{}", returns_closure2()(1));
}

fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

fn returns_closure() -> impl Fn(i32) -> i32 {
  |x| x + 1
}

fn returns_closure2() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

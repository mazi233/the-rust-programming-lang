pub fn main() {
  let x = 5;
  let y: Kilometers = 5;

  println!("x + y = {}", x + y);

  // let f: Thunk = Box::new(|| println!("hi"));
  takes_long_type(returns_long_type());

  generic(1);
  generic2("sdsd");
  generic3(&Box::new(11));
}

type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(_f: Thunk) {}
fn returns_long_type() -> Thunk {
  Box::new(|| println!("hi"))
}

fn generic<T>(t: T) {}
fn generic2<T: Sized>(t: T) {}
fn generic3<T: ?Sized>(t: &T) {}

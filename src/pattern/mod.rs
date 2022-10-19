pub fn main() {
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }
  println!("at the end: x = {:?}, y = {:?}", x, y);

  let x = 1;
  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  let x = 5;
  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }

  let x = 'c';
  match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }

  let p = Point {x: 0, y: 7};
  let Point {x, y} = p;
  println!("{x} {y}");

  match p {
    Point{x, y: 0} => println!("On the x axis at {}", x),
    Point{x: 0, y} => println!("On the y axis at {}", y),
    Point{x, y} => println!("On neither axis: ({}, {})", x, y),
  }

  let s = Some(String::from("Hello!"));
  // 1. 以下划线开头的未使用变量仍然会绑定值，它可能会获取值的所有权
  if let Some(_s) = s {
    println!("found a string");
  }
  // 2. 单独使用下划线不会绑定值
  // if let Some(_) = s {
  //   println!("found a string");
  // }
  // println!("{:?}", s);
  
  let origin = Point { x: 0, y: 0 };
  match origin {
    // 用 .. 忽略剩余值
    Point { x, .. } => println!("x is {}", x),
  }

  let numbers = (2, 4, 8, 16, 32);
  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }

  // 匹配守卫提供的额外条件
  // 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
  // 匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。
  let num = Some(4);
  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  let x = Some(5);
  let y = 10;
  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {}", n),
    _ => println!("Default case, x = {:?}", x),
  }
  println!("at the end: x = {:?}, y = {}", x, y);

  let x = 4;
  let y = false;
  match x {
    // (4 | 5 | 6) if y => ...
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }

  let msg = Message::Hello { id: 5 };
  match msg {
    // at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
    Message::Hello {
      id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Message::Hello { id: 10..=12 } => {
      println!("Found an id in another range");
    }
    Message::Hello { id } => println!("Found some other id: {}", id),
  }
}

enum Message {
  Hello { id: i32 },
}

struct Point {
  x: i32,
  y: i32,
}

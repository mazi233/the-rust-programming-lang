pub fn other() {
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

  let x = x + 1;
  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }
  println!("The value of x is: {x}");

  let x = 255u8;
  println!("{x}");

  let _x = 2.0;
  let _z = 'ℤ';

  let tup = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("{} {} {} {} {} {}", tup.0, tup.1, tup.2, x, y, z);

  let a = [3; 5];
  println!("{:?}", a);

  // let mut index = String::new();

  // std::io::stdin()
  //   .read_line(&mut index)
  //   .expect("Failed to read line");
  
  // let index: usize = index
  //   .trim()
  //   .parse()
  //   .expect("Index entered was not a number");

  // if index >= a.len() {
  //   panic!("err index");
  // }

  // let element = a[index];
  // println!("The value of the element at index {index} is: {element}");

  other_function(5);
  print_labeled_measurement(5, 'h');

  let _y = {
    let x = 3;
    x + 1
  };

  println!("{}", plus_one(4));

  // loop {
  //   println!("again");
  // }

  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };
  println!("The result is {result}");

  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

  let mut number = 3;
  while number != 0 {
    println!("{number}!");

    number -= 1;
  }
  println!("LIFTOFF!!!");

  let a = [10, 20, 30, 40, 50];
  for element in a {
    println!("the value is: {element}");
  }

  for number in (1..=4).rev() {
    println!("{number}")
  }
  println!("LIFTOFF!!!");

  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{s}");

  let mut s1 = String::from("hello");

  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);
  
  change(&mut s1);
  println!("s1 is {}", s1);
  // let r1 = &mut s1;
  // let r2 = &mut s1;
  // println!("{} {}", r1, r2);

  // let r1 = &s1;
  // let r2 = &s1;
  // let r3 = &mut s1;
  // println!("{} {} {}", r1, r2, r3);

  let _reference_to_nothing = dangle();

  let mut s = String::from("hello world");
  let _word = first_word(&s);
  s.clear();
  
  let s = String::from("hello world");
  let hello = &s[0..=5];
  let world = &s[6..11];
  println!("{}{}", hello, world);

  println!("{}", first_word2(&s));

  let a = [1,2,3,4,5];
  let slice = &a[1..3];
  assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }
  s.len()
}

fn first_word2(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  &s[..]
}

fn dangle() -> String {
  let s = String::from("hello");
  s
}

fn change(s: &mut String) {
  s.push_str(", world!")
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn other_function(x: i32) {
  println!("The value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
  return  x + 1;
}
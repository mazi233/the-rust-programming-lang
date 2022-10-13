pub fn main() {
  let mut v: Vec<i32> = Vec::new();
  let mut v = vec![1, 2, 3, 4];
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(9);

  let third = v[2];
  println!("The third element is {third}");
  
  let third = v.get(2).unwrap();
  println!("The third element is {}", third);

  // let does_not_exist = v[100];
  if let Some(does_not_exist) = v.get(100) {
    println!("{}", does_not_exist);
  } else {
    println!("laji");
  }

  let mut v = vec![1, 2, 3, 4, 5];
  let first = &v[0];
  println!("The first element is: {}", first);
  v.push(6);
  println!("v is: {:?}", v);

  let mut v = vec![100, 32, 57];
  for i in &mut v {
    // println!("{}", i);
    *i += 50;
  }

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {}", s2);

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

  let usera = User {age: 12};
  let userb = User {age: 13};
  println!("user age added is {}", usera + userb);

  use std::collections::HashMap;
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];
  let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
  println!("scores is {:?}", scores);

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);

  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);
  println!("{:?}", scores);

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map);
}

struct User {
  age: i32,
}

impl std::ops::Add for User {
    type Output = i32;

    fn add(self, rhs: Self) -> Self::Output {
      self.age + rhs.age
    }
}

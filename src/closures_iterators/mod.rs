use std::collections::HashMap;
// use std::thread;
// use std::time::Duration;

pub fn main() {
  let x = vec![1, 2, 3];
  let equal_to_x = |z| z == x;
  println!("can't use x here: {:?}", x);
  let y = vec![1, 2, 3];
  assert!(equal_to_x(y));

  
  let x = 4;
  let equal_to_x = |z| z == x;
  let y = 4;
  assert!(equal_to_x(y));

  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  let mut expensive_closure = MapCacher::new(|num| {
    println!("calculating slowly...");
    // thread::sleep(Duration::from_secs(2));
    num * 2
  });
  println!("{}", expensive_closure.value(1));
  println!("{}", expensive_closure.value(1));
  println!("{}", expensive_closure.value(2));
  println!("{:?}", expensive_closure.map);

  generate_workout(simulated_user_specified_value, simulated_random_number);


  let v1 = vec![1,2,3];
  let v1_iter = v1.iter();
  for val in v1_iter.clone() {
    println!("Got: {}", val);
  }
  let total: i32 = v1_iter.clone().sum();
  assert_eq!(6, total);
  let v2: Vec<i32> = v1_iter.clone().map(|x| x + 1).collect();
  assert_eq!(v2, vec![2, 3, 4]);

  let filters_by_size = || {
    let shoes = vec![
      Shoe{size: 10, style: String::from("sneaker"),},
      Shoe{size: 13, style: String::from("sandal"),},
      Shoe{size: 10, style: String::from("boot"),},
    ];
    let in_my_size = shoes_in_size(shoes, 10);
    assert_eq!(in_my_size, vec![
      Shoe{size: 10, style: String::from("sneaker"),},
      Shoe{size: 10, style: String::from("boot"),},
    ])
  };
  filters_by_size();

  let mut counter = Counter::new();
  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);

  let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
  assert_eq!(18, sum);
  println!("{:?}", Counter::new().zip(Counter::new().skip(1)));
}

#[derive(Debug)]
struct Counter {
  count: u32,
}
impl Counter {
  fn new() -> Self {
    Self { count: 0 }
  }
}
impl Iterator for Counter {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn generate_workout(intensity: u32, random_number: u32) {
  // let expensive_closure = |num| {
  //   println!("calculating slowly...");
  //   thread::sleep(Duration::from_secs(2));
  //   num
  // };

  let mut expensive_closure = Cacher::new(|num| {
    println!("calculating slowly...");
    // thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_closure.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      expensive_closure.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_closure.value(intensity)
      );
    }
  }
}

struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher { calculation, value: None, }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

struct MapCacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  map: HashMap<u32, u32>,
}

impl<T> MapCacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> MapCacher<T> {
    MapCacher { calculation, map: HashMap::new() }
  }

  fn value(&mut self, arg: u32) -> u32 {
    if let Some(v) = self.map.get(&arg) {
      *v
    } else {
      let v = (self.calculation)(arg);
      self.map.insert(arg, v);
      v
    }
  }
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//   println!("calculating slowly...");
//   thread::sleep(Duration::from_secs(2));
//   intensity
// }

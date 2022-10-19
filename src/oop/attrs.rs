pub fn main() {
  let mut ac = AverageCollection::new();
  ac.add(1);
  ac.add(2);
  ac.add(3);
  ac.add(4);
  ac.remove();
  println!("{}", ac.average());
}

pub struct AverageCollection {
  coll: Vec<i32>,
  average: f64,
}

impl AverageCollection {
  pub fn new() -> Self {
    Self { coll: vec![], average: 0.0 }
  }

  pub fn add(&mut self, v: i32) {
    self.coll.push(v);
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.coll.pop();
    match result {
      Some(v) => {
        self.update_average();
        Some(v)
      },
      None => None,
    }
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  fn update_average(&mut self) {
    let total: i32 = self.coll.iter().sum();
    self.average = total as f64 / self.coll.len() as f64;
  }
}

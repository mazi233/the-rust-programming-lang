pub fn main() {
  assert_eq!(2 + 2, 4);
}

#[cfg(test)]
mod tests {
  #[test]
  fn exploration() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn another() {
    // panic!("Make this test fail");
  }

  use crate::rectangles::Rectangle;

  use super::{add_two, greeting, Guess};
  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };
    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };
    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn it_adds_two() {
    assert_eq!(4, add_two(2));
    assert_ne!(5, add_two(2));
  }

  #[test]
  fn greeting_contains_name() {
    let result = greeting("mazi");
    assert!(
      result.contains("mazi"),
      "Greeting did not contain name, value was `{}`",
      result,
    )
  }

  #[test]
  #[should_panic(expected="Guess value must be less than or equal to 100")]
  fn greater_than_100() {
    Guess::new(200);
  }

  pub fn out_it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

  #[test]
  #[ignore]
  fn it_works() -> Result<(), String> {
    out_it_works()?;
    Ok(())
    // Ok("mmm".to_string())
  }
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}

pub fn greeting(name: &str) -> String {
  format!("Hello {}", name)
}

pub struct Guess {
  value: i32,
}
impl Guess {
  pub fn new(value: i32) -> Self {
    // if value < 1 || value > 100 {
    //   panic!("Guess value must be between 1 and 100, got {}.", value);
    // }
    if value < 1 {
      panic!(
        "Guess value must be greater than or equal to 1, got {}.",
        value
      );
    } else if value > 100 {
      panic!(
        "Guess value must be less than or equal to 100, got {}.",
        value
      );
    }
    Self { value }
  }
}

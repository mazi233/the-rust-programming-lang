use std::fs::File;
use std::io::{ErrorKind, Read};

pub fn main() {
  let f = File::open("hello.txt");

  // let f = match f {
  //   Ok(file) => file,
  //   Err(error) => match error.kind() {
  //     ErrorKind::NotFound => match File::create("hello.txt") {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Porblem creating the file: {:?}", e),
  //     },
  //     other_error => panic!("Porblem opening the file: {:?}", other_error),
  //   },
  // };
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Porblem creating the file: {:?}", error);
      })
    } else {
      panic!("Porblem opening the file: {:?}", error);
    }
  });
  println!("{:?}", f);

  println!("{:?}", last_char_of_first_line(read_username_from_file().unwrap().as_str()));
}

// fn read_username_from_file() -> Result<String, std::io::Error> {
//   let mut f = File::open("hello.txt")?;
//   let mut s = String::new();
//   f.read_to_string(&mut s)?;
//   Ok(s)
// }

fn read_username_from_file() -> Result<String, std::io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?
    .read_to_string(&mut s)?;
  Ok(s)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}

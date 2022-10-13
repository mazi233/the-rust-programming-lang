enum IPAddrKind {
  V4,
  V6,
}

struct IPAddr {
  kind: IPAddrKind,
  address: String,
}

enum IPAddr2 {
  V4(String),
  V6(String),
}

pub fn main() {
  let home = IPAddr {
    kind: IPAddrKind::V4,
    address: "127.0.0.1".to_string(),
  };
  let loopback = IPAddr {
    kind: IPAddrKind::V6,
    address: "::1".to_string(),
  };
  let home = IPAddr2::V4("127.0.0.1".to_string());
  let loopback = IPAddr2::V4("::1".to_string());

  let x: i8 = 2;
  let y: Option<i8> = Some(3);

  println!("x + y = {}", x + y.unwrap());

  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
  }

  let config_max = Some(3u8);
  if let Some(max) = config_max {
    println!("the max number is {max}");
  }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
  println!("player moved {} step(s)", num_spaces);
}

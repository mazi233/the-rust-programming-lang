pub fn _main() {
  // other::other();
  let user1 = build_user("mazi@qq.com".to_string(), "mazi".to_string());

  // 请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据，就像我们在“变量与数据交互的方式（一）：移动”部分讲到的一样。
  // 在这个例子中，我们在创建 user2 后不能再使用 user1，因为 user1 的 username 字段中的 String 被移到 user2 中。
  // 如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，
  // 那么 user1 在创建 user2 后仍然有效。active 和 sign_in_count 的类型是实现 Copy trait 的类型，
  // 所以我们在“变量与数据交互的方式（二）：克隆” 部分讨论的行为同样适用
  let user2 = User{
    email: "mazi2@qq.com".to_string(),
    username: "mazi2".to_string(),
    ..user1
  };
  println!("{:?} {:?}", user1, user2);

  let _black = Color(0, 0, 0);
  let _origin = Point(0, 0, 0);
  let _subject = AlwaysEqual;
  let _subject2 = AlwaysEqual;
  println!("{:?} {:?}", _subject, _subject2);

  let user2 = User2{
    email: "mazi2@qq.com",
    username: "mazi2",
    active: true,
    sign_in_count: 1,
  };
  println!("{:?}", user2);
}

#[derive(Debug)]
pub struct User {
  pub active: bool,
  pub username: String,
  pub email: String,
  pub sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

pub struct Color(pub i32, pub i32, pub i32);
pub struct Point(pub i32, pub i32, pub i32);

#[derive(Debug)]
pub struct AlwaysEqual;

#[derive(Debug)]
pub struct User2<'a> {
  pub active: bool,
  pub username: &'a str,
  pub email: &'a str,
  pub sign_in_count: u64,
}

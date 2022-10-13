pub fn main() {
  let width1 = 30;
  let height1 = 50;
  let rect1 = (30, 50);

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1, height1)
  );
  println!(
    "The area of the rectangle is {} square pixels.",
    area2(rect1)
  );

  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };
  println!(
    "The area of the rectangle is {} square pixels.",
    area3(&rect1),
  );
  println!("{:#?}", rect1);
  dbg!(&rect1);
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
  println!("{:?}", Rectangle::square(10).area());
}

#[derive(Debug)]
pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

impl Rectangle {
  fn square(size: u32) -> Self {
    Self { width: size, height: size }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }

  pub fn can_hold(&self, rect2: &Self) -> bool {
    self.width > rect2.width && self.height > rect2.height
  }
}

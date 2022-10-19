pub trait Draw {
  fn draw(&self);
}

// trait object
pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}
impl Draw for Button {
  fn draw(&self) {
    
  }
}

// 这与定义使用了带有 trait bound 的泛型类型参数的结构体不同。泛型类型参数一次只能替代一个具体类型，而 trait object 则允许在运行时替代多种具体类型
// trait bound
pub struct Screen2<T: Draw> {
  pub components: Vec<T>,
}
impl<T> Screen2<T>
where
  T: Draw
{
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

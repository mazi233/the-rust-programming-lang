pub fn main() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("{}", *r1);
    println!("{}", *r2);
  }

  let address = 0x012345usize;
  let _r = address as *const i32;
  // unsafe {
  //   println!("{}", *r);
  // }

  unsafe {
    dangerous();
  }

  let mut v = vec![1, 2, 3, 4, 5, 6];
  // let r = &mut v[..];
  // let (a, b) = v.split_at_mut(3);
  let (a, b) = split_at_mut(&mut v, 3);
  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);

  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }

  println!("name is: {}", HELLO_WORLD);

  add_to_count(3);
  unsafe {
    println!("COUNTER: {}", COUNTER);
  }
}

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  // let len = slice.len();
  // assert!(mid <= len);
  // (&mut slice[..mid], &mut slice[mid..])

  let len = slice.len();
  let ptr = slice.as_mut_ptr();
  assert!(mid <= len);
  unsafe {
    use std::slice;
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
  }
}

extern "C" {
  fn abs(input: i32) -> i32;
}

// 从其它语言调用 Rust 函数
// extern 的使用无需 unsafe
#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";

// 常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
  unsafe { COUNTER += inc }
}

// 实现不安全 trait
unsafe trait Foo {
  
}
unsafe impl Foo for i32 {
  
}

// 访问联合体中的字段

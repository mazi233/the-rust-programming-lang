use the_rust_programming_lang;
mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, the_rust_programming_lang::add_two(2));
}
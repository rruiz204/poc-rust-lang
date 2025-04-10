pub fn showcase() {
  println!("=== Integers");
  let integer: i32 = 190;
  println!("integer: {}", integer);

  println!("=== Floats");
  let float: f64 = 3.14;
  println!("float: {}", float);

  println!("=== Booleans");
  let maybe: bool = true;
  println!("boolean: {}", maybe);

  println!("=== Characters");
  let letter: char = 'R';
  println!("letter: {}", letter);

  println!("=== Strings");
  let string: String = String::from("hello world");
  println!("string: {}", string);

  println!("=== Slices");
  let slice: &str = &string[0..5];
  println!("slice: {}", slice);
}
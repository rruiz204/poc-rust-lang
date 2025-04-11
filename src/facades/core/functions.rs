pub fn showcase() {
  println!("=== Clousures");
  let sum = |a: i32, b: i32| a + b;
  
  let result: i32 = sum(10, 2);
  println!("The result of the sum is: {}", result);
}
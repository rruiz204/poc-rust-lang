pub fn showcase() {
  println!("=== Vectors");
  let mut numbers: Vec<i32> = vec![1, 2, 3];

  numbers.push(4);
  numbers.push(5);
  numbers.push(6);

  numbers.reverse();
  for number in numbers {
    println!("number: {}", number);
  }
}
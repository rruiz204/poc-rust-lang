use std::collections::HashMap;

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


  println!("=== HashMaps");
  let mut scores: HashMap<String, i32> = HashMap::new();

  scores.insert(String::from("R"), 100);
  scores.insert(String::from("O"), 200);
  scores.insert(String::from("N"), 300);

  if let Some(score) = scores.get("R") {
    println!("Score: {}", score);
  }
}
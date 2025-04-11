pub fn showcase() {
  println!("=== While loop");
  let mut counter: i32 = 0;

  while counter <= 5 {
    println!("The value of the counter is: {}", counter);
    counter += 1;
  }


  println!("=== For loop");
  let fruits: Vec<String> = vec![
    String::from("kiwi"),
    String::from("apple"),
    String::from("banana"),
  ];

  for fruit in fruits {
    println!("fruit: {}", fruit);
  }
}
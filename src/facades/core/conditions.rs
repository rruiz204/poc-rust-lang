pub fn showcase() {
  println!("=== Condition Expressions");
  let maybe: bool = true;

  if maybe && 1 > 0 {
    println!("The maybe value is true");
  }


  println!("=== Ternary Operator");
  let is_adult: bool = if 17 >= 18 { true } else { false };
  println!("permission: {}", is_adult);


  println!("=== If Let Expressions");
  let something: Option<i32> = Some(100);

  if let Some(value) = something {
    println!("the value is not null: {}", value);
  }
}
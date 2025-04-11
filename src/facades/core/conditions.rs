pub fn showcase() {
  println!("=== Condition Expressions");
  let maybe: bool = true;

  if maybe && 1 > 0 {
    println!("The maybe value is true");
  }


  println!("=== Ternary Operator");
  let is_adult: bool = if 17 >= 18 { true } else { false };
  println!("permission: {}", is_adult);
}
fn is_even(number: i32) -> Result<i32, String> {
  if number % 2 == 0 {
    Ok(number)
  } else {
    Err(String::from("The value is not even"))
  }
}

fn facade(number: i32) -> Result<i32, String> {
  let resolve = is_even(number)?;
  Ok(resolve)
}

pub fn showcase() {
  println!("=== Basic Exception Handling");
  let resolve = is_even(10);

  if let Ok(number) = resolve {
    println!("All work corretly: {}", number);
  }

  let resolve = is_even(11);
  if let Err(error) = resolve {
    println!("We found an error: {}", error);
  }


  println!("=== Exception Handling with '?' Operator");
  let resolve = facade(13);

  if let Ok(number) = resolve {
    println!("All work corretly: {}", number);
  }
}
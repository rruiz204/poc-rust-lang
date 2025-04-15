fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

pub fn showcase() {
  let s1 = "abcd";
  let result: &str;

  {
    let s2 = "xyz";
    result = longest(&s1, &s2);
    println!("Th largest is: {}", result);
  } // s2 dies here, so result does too
}
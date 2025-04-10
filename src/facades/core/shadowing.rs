pub fn showcase() {
  println!("=== Shadowing variables");

  let shdw_var: i32 = 120;
  println!("before shadowing: {}", shdw_var);

  let shdw_var: bool = true;
  println!("after shadowing: {}", shdw_var);
}
pub fn showcase() {
  println!("=== Inmutable variables");
  let imt_number: i32 = 10;
  println!("inmutable variable: {}", imt_number);


  println!("=== Mutable variables");
  
  let mut mtb_number: i32 = 90;
  println!("before mutable variable: {}", mtb_number);

  mtb_number = 100;
  println!("after mutable variable: {}", mtb_number);
}

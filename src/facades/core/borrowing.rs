pub fn showcase() {
  println!("=== Inmutable references in Borrowing");
  let imt_string: String = String::from("imutable text");

  let first_imt_ref_string: &String = &imt_string;
  println!("first inmutable reference: {}", first_imt_ref_string);

  let second_imt_ref_string: &String = &imt_string;
  println!("second inmutable reference: {}", second_imt_ref_string);


  println!("=== Mutable references in Borrowing");

  let mut mtb_string: String = String::from("mutable text");
  println!("before mutable string: {}", mtb_string);

  let unique_mtb_ref_string: &mut String = &mut mtb_string;
  unique_mtb_ref_string.push_str(" with more text");

  println!("after mutable string: {}", mtb_string);
}
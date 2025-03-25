pub fn facade() {
    println!("=== Inmutable variables");
    let imt_number: i32 = 10;
    println!("inmutable variable: {}", imt_number);


    println!("=== Mutable variables");
    let mut mtb_number: i32 = 90;
    println!("before mutable variable: {}", mtb_number);

    mtb_number = 100;
    println!("after mutable variable: {}", mtb_number);


    println!("=== Shadowing variables");
    let shdw_number: i32 = 120;

    let shdw_number: i32 = shdw_number + 10;
    println!("shadowing variable: {}", shdw_number);


    println!("=== Inmutable references in Borrowing");
    let imt_string: String = String::from("imutable text");

    let imt_ref_string: &String = &imt_string;
    println!("inmutable reference: {}", imt_ref_string);


    println!("=== Mutable references in Borrowing");
    let mut mtb_string: String = String::from("mutable text");
    println!("before mutable string: {}", mtb_string);

    let mtb_ref_string: &mut String = &mut mtb_string;
    mtb_ref_string.push_str(" with more text");
    println!("after mutable string: {}", mtb_string);
}
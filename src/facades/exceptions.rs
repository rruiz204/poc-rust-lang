fn divide(num_a: i32, num_b: i32) -> Result<i32, String> {
    if num_b == 0 {
        Err(String::from("zero"))
    } else {
        Ok(num_a / num_b)
    }
}

pub fn facade() {
    if let Ok(value) = divide(10, 5) {
        println!("The result is: {}", value);
    }
}
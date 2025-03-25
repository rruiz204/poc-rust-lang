fn sum(num_a: i32, num_b: i32) -> i32 {
    num_a + num_b
}

pub fn facade() {
    println!("=== Functions with parameters");
    let result: i32 = sum(10, 19);
    println!("The result is: {}", result);
}
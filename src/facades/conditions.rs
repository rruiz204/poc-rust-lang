pub fn facade() {
    println!("=== Simple conditions");
    let number: i32 = 19;

    if number > 5 {
        println!("The number is bigger than 5");
    } else {
        println!("The number is shorter than 5");
    }


    println!("=== Alternative to ternary operator");
    let number: i32 = if true { 10 } else { 5 };
    println!("Simple exampel of ternary operator: {}", number);
}
pub fn facade() {
    println!("=== Conditions for control");
    if 84 % 2 == 0 { println!("is an even number"); }

    
    println!("=== Alternative to ternary");
    let number: i32 = if true { 19 } else { 10 };
    println!("number value: {}", number);


    println!("=== Using while loop");
    let mut counter: i32 = 0;
    while counter <= 5 {
        println!("while counter value: {}", counter);
        counter += 1;
    }

    println!("=== Using for loop");
    let numbers: [i32; 3] = [1, 2, 3];
    for item in numbers.iter() {
        println!("for item value: {}", item);
    }
}
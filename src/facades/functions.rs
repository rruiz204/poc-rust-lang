fn sum(num_a: i32, num_b: i32) -> i32 {
    num_a + num_b
}

fn concat(mut text_a: String, text_b: String) -> String {
    text_a.push_str(&text_b);
    text_a
}

pub fn facade() {
    println!("=== Functions with parameters");
    let result: i32 = sum(10, 19);
    println!("The result is: {}", result);


    println!("=== Functions with mutable parameters");
    let text_a: String = String::from("Hello ");
    let text_b: String = String::from("world!!!");

    println!("text_a is invalid because it move to concat scope");
    let result: String = concat(text_a, text_b);
    println!("The result is: {}", result);
}
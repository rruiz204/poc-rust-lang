pub fn facade() {
    println!("=== Simple matching use");
    let number: i32 = 3;
    match number {
        1 => println!("first"),
        2 => println!("second"),
        3 => println!("third"),
        _ => println!("other number")
    }
}
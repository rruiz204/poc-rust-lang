pub fn facade() {
    let numbers: Vec<i32> = vec![1, 2, 3];
    for item in &numbers {
        println!("Item value: {}", item);
    }

    let first: Option<&i32> = numbers.get(0);
    if let Some(value) = first {
        println!("The first item of vector is: {}", value);
    }
}
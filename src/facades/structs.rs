struct User {
    name: String,
    email: String,
    password: String,
}

impl User {
    fn new(name: String, email: String, password: String) -> Self {
        Self { name, email, password }
    }

    fn showcase(&self) {
        println!("The name of user1 is: {}", self.name);
        println!("The email of user1 is: {}", self.email);
        println!("The password of user1 is: {}", self.password);
    }
}

pub fn facade() {
    println!("=== Using Structs");
    let user1: User = User::new(
        String::from("example"),
        String::from("example@test.com"),
        String::from("12345678")
    );
    user1.showcase();
}
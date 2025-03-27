struct User {
    name: String,
    email: String,
    password: String,
}

impl User {
    fn showcase(&self) {
        println!("The name of user1 is: {}", self.name);
        println!("The email of user1 is: {}", self.email);
        println!("The password of user1 is: {}", self.password);
    }
}

pub fn facade() {
    println!("=== Using Structs");
    let user1: User = User {
        name: String::from("example"),
        email: String::from("example@test.com"),
        password: String::from("12345678"),
    };
    user1.showcase();
}
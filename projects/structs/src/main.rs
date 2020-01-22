use std;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {

    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }

}

// derive debug annotation is for the rectangle struct, it allows us to print
// the rectangle struct using the println! macro and {:?} or {:#?} for pretty print
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// essentially lets the code in the curly brackets know that self is of type Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // function to see if rectangle 2 fits in rectangle 1
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn user_example() {

    let user1 = User {
        email: String::from("person@email.com"),
        username: String::from("Ausername"),
        active: true,
        sign_in_count: 100,
    };

    let user2 = build_user(String::from("dogs@dogs.com"),
                           String::from("IAMACAT"));

    let user3 = User {
        email: String::from("An@email.com"),
        username: String::from("Name"),
        ..user2
    };
}

fn main() {

    let rect1 = Rectangle{ width: 30, height: 40 };

    println!("The rectangle is {:#?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());
}

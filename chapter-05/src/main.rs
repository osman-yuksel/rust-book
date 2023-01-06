// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// https://doc.rust-lang.org/book/ch05-02-example-structs.html
use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.
// To do that, we add the outer attribute #[derive(Debug)] just before the struct definition.
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

fn perimeter(rectangle: &Rectangle) -> u32 {
    2 * (rectangle.width + rectangle.height)
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername123"),
        ..user1
    };

    println!("{}, {}, {}, {}", user1.username, user1.email, user1.active, user1.sign_in_count);
    println!("{}, {}, {}, {}", user2.username, user2.email, user2.active, user2.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}, {}", black, origin);

    let rect1 = Rectangle {
        width: 40,
        height: 60,
    };
    
    println!("{}", perimeter(&rect1));
    
    // Debug printing
    println!("{:?}", rect1);
    // Takes ownership of variable
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("{}", rect2.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(5);
    println!("{:?}", rect4);

}

// println!() can't print these structs by default.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}
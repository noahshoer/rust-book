fn main() {
    let mut user1 = build_user("baz", String::from("baz@foo.com"));

    user1.email = String::from("baz@bar.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // .. syntax gets all other fields from user2
    let _user3 = User {
        username: String::from("baa"),
        ..user2
    };
    // user2 is dead now since it was moved to user3 (would not have been
    // if only the bool and u64 values were the ones requested as they
    // can be copied. We can use the non moved field though (username)
    println!("Username: {}", user2.username);

    let black = Color(0, 0, 0);
    let Color(_r, _g, _b) = black;

    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {rect1:#?}");

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(10);
    println!("Square is {sq:#?}");
}

// Add debug printing
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: &str, email: String) -> User {
    // If the parameter is the same as the field, you don't need
    // N-V syntax
    User {
        active: true,
        username: String::from(username),
        email,
        sign_in_count: 1,
    }
}

// Can define a tuple struct, which serves to name the tuple without requiring
// N-V pairs
struct Color(i32, i32, i32);

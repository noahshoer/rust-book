use generics::aggregator::{SocialPost, Summary};
use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p4 = Point { x: 5.0, y: 3.0 };
    println!("p4 dist from origin = {}", p4.distance_from_origin());

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new social post: {}", post.read());
    notify(&post);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "boo");
        println!("The longest string is {result}");
        // Result is dropped at the end of the inner scope since string2 has the shorter
        // lifetime and is dropped there
    }
}

// Lifetime annotations to instruct the borrow checker the output lives
// at least as long as the two inputs. It is the smaller of the two of the lifetimes
// of x and y. You only need to annotate the inputs that have an effect on the output
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Trait impl syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Can specify multiple trait bound types with +
// Can also use a where class to keep the trait bindings out of the signature
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Definition for a specific type
impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Create a Point with type for x from 1, and type for y from other
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

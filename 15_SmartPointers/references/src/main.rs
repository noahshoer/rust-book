#![allow(dead_code)]

// Needs the Box to calculate size from pointer type
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::rc::Rc;
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

use crate::List::{Cons, Nil};

// Sample pointer that doesn't store on the heap
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Override deref (*) for MyBox
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    // Under the hood, * calls this to get a reference to dereference like so:
    // *(x.deref())
    //
    // Rust also uses deref coercion, and will call deref as many times as needed
    // to get the appropriate reference type at compile time
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Cleanup whenever the box goes out of scope
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

fn main() {
    // Box stores data on the heap, the box is on the stack
    // much like a C style pointer
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print_list(&list);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // Clone does a reference count increase, not a deep copy
    let _b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn print_list(list: &List) {
    match list {
        Cons(val, next) => {
            print!("{val} ");
            print_list(next);
        }
        Nil => {
            println!();
        }
    }
}


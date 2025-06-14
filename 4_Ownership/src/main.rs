fn main() {
    // String literals are immutable since they are a slice pointing
    // to the specific point in the binary
    let s = "Hello";
    let s2 = String::from(s); // heap allocated 
    // String holds a pointer, length, and capacity
    let s3 = s2; // Heap data not copied, just p, l, and c
        // At this point, s2 is no longer valid since there can only
        // be one owner
        // Thus, rust moves instead of shallow copies, rust never 
        // deep copies by default for heap allocated data
        // Can use the clone() method to deep copy

        // Types can implement the Copy or Drop trait:
        // Copy allows for copies, Drop fopr moves

    let mut s4 = takes_and_gives_back(s3); // s2 is moved into
                                // takes_and_gives_back, which also
                                // moves its return value into s3
    
    let len = calculate_length(&mut s4);

    println!("Length of string is {len}, First word is: {}", first_word(&s4));

    takes_ownership(s4);    // s's value moves into the function...
                                        // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);     // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward
}


// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// References are immutable by default, and you can only have one
// mutable reference at a time
fn calculate_length(s: &mut String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
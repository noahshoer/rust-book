use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    let v = vec![1, 2, 3];
    // Move into the closure to force it to take ownership of v, otherwise
    // we'll get a compiler error
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle2.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Force the spawned thread to run to completion by blocking main from completing
    // If we put this before the main for loop, the spawned thread will run to completion
    // before the main thread starts
    handle.join().unwrap();
}

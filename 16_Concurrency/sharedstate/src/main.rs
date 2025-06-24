use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let m = Mutex::new(5);

    {
        // lock() acquires the lock, unwrap to panic if another thread holding the lock panicked
        let mut num = m.lock().unwrap();
        *num = 6;

        // lock automatically released when the MutexGuard goes out of scope
    }

    println!("m = {m:?}");

    // Arc is an atomic Rc, and Mutex provides interior mutability like a RefCell
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
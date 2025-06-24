use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // Create a channel and get the sender and receiver (Multiple Producer Single Consumer)
    let (tx, rx) = mpsc::channel();

    // Get another transmitter by cloning it
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Move the transmitter into the thread
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv blocks execution until it gets a value, try_recv will not block and return a Result
    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {received}");
    }
}
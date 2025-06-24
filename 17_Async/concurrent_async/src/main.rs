use std::{pin::{pin, Pin}, time::Duration};

fn main() {
    // trpl::run(async {
    //     let handle = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("hi number {i} from the second task!");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }

    //     // assign task to a handle and call handle.await.unwrap() to let spawned task run to completion
    //     handle.await.unwrap();

    //     // Await both futures finishing
    //     let fut1 = async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     let fut2 = async {
    //         for i in 1..5 {
    //             println!("hi number {i} from the second task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     // Unlike threads, you see the same output everytime from trpl::join since it checks
    //     // each future equally
    //     trpl::join(fut1, fut2).await;
    // });

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // Put transmitter and receiver into a future to not be serial send/receive.
        // Do so by move to make sure it gets dropped so that the receiver can know the channel's other
        // end was closed. Can also clone to force multiple futures
        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
        trpl::join_all(futures).await;
    });

    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        // race is not fair, will always run arguments in order they're passed in instead of
        // randomly deciding
        trpl::race(slow, fast).await;
    });
}
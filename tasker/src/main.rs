use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Thread ID: {:?}", thread::current().id());
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Using multiple producers and a single consumer
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let new_tx = tx.clone();
        thread::spawn(move || {
            for j in 0..=10 {
                new_tx
                    .send(format!(
                        "Data: {:3} from thread ID: {:?}",
                        i * j,
                        thread::current().id()
                    ))
                    .unwrap();
            }
        });
    }

    // The last transmitter is dropped so that the receiver can finish
    drop(tx);

    for received in rx {
        println!("Got: {}", received);
    }
}

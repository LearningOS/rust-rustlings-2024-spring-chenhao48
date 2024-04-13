// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: &Arc<Queue>, tx: mpsc::Sender<u32>, data: &[u32]) {
    for val in data {
        println!("sending {:?}", val);
        tx.send(*val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length;

    let queue_clone = Arc::clone(&queue);
    let tx_clone = mpsc::Sender::clone(&tx);
    let t1 = thread::spawn(move || send_tx(&queue_clone, tx_clone, &queue_clone.first_half));

    let queue_clone = Arc::clone(&queue);
    let tx_clone = mpsc::Sender::clone(&tx);
    let t2 = thread::spawn(move || send_tx(&queue_clone, tx_clone, &queue_clone.second_half));

    t1.join().expect("Thread 1 panicked");
    t2.join().expect("Thread 2 panicked");

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
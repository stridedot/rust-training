use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let value = tx.clone();
        thread::spawn(move || producer(i, value));
    }

    for received in rx {
        println!("Got: {}", received);
    }
}

fn producer(i: usize, tx: mpsc::Sender<usize>) {
    tx.send(i).unwrap();
}

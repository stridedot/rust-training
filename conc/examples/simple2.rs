use std::sync::Arc;
use std::{sync::Mutex, thread};

fn main() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let m = m.clone();
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *m.lock().unwrap());
}

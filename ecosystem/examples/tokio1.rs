use std::{thread, time::Duration};

use tokio::runtime::Builder;

fn main() {
    let handle = thread::spawn(move || {
        // execute future
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        rt.spawn(async {
            println!("Future 1!");
        });

        rt.spawn(async {
            println!("Future 2!");
        });

        rt.block_on(async {
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
    });

    handle.join().unwrap();
}

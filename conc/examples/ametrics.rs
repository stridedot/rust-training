use std::{thread, time::Duration};

use conc::metrics::amap::AmapMetrics;

fn main() {
    let metrics = AmapMetrics::new();
    metrics.inc("key", 1);
    metrics.inc("key", 2);
    metrics.dec("key", 1);
    metrics.read();

    // 多线程
    let metrics = AmapMetrics::new();
    let mut handles = vec![];

    for i in 0..2 {
        let m = metrics.clone();
        let handle = thread::spawn(move || {
            for _ in 0..500 {
                m.dec("count", 1);
            }

            println!("线程 {} 完成 dec", i);
        });
        handles.push(handle);
    }

    thread::sleep(Duration::from_millis(1000));

    // 四个线程执行 inc
    for i in 0..4 {
        let m = metrics.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                m.inc("count", 1);
            }
            println!("线程 {} 完成 inc", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

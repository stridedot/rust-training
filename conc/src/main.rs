use std::{thread, time::Duration};

use conc::metrics::cmap::CmapMetrics;

fn main() {
    // 单线程操作
    let metrics = CmapMetrics::<usize>::new();
    metrics.inc("a", 1);
    metrics.inc("a", 2);
    metrics.dec("a", 1);
    println!("{:?}", metrics.data);

    // 多线程操作
    let metrics = CmapMetrics::<isize>::new();

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

    metrics.read();
    thread::sleep(Duration::from_millis(100));
}

use std::sync::{
    Arc,
    atomic::{AtomicI64, Ordering},
};

use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    pub data: Arc<DashMap<String, Arc<AtomicI64>>>,
}

impl AmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: &str, value: i64) {
        let counter = self
            .data
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(AtomicI64::new(0)));
        counter.fetch_add(value, Ordering::Relaxed);
    }

    pub fn dec(&self, key: &str, value: i64) {
        let counter = self
            .data
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(AtomicI64::new(0)));
        counter.fetch_sub(value, Ordering::Relaxed);
    }

    pub fn read(&self) {
        for entry in self.data.iter() {
            let key = entry.key().clone();
            let counter = entry.value().clone();
            println!("{key}: {:?}", counter.load(Ordering::Relaxed));
        }
    }
}

impl Default for AmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

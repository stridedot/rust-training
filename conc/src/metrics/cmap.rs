use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
    sync::Arc,
};

use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct CmapMetrics<T> {
    pub data: Arc<DashMap<String, T>>,
}

impl<T> CmapMetrics<T>
where
    T: Clone + Debug + Default + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: &str, value: T) {
        let mut counter = self.data.entry(key.to_string()).or_default();
        *counter += value;
    }

    pub fn dec(&self, key: &str, value: T) {
        let mut counter = self.data.entry(key.to_string()).or_default();
        *counter -= value;
    }

    pub fn read(&self) {
        for entry in self.data.iter() {
            println!("{}: {:?}", entry.key(), entry.value());
        }
    }
}

impl<T> Default for CmapMetrics<T>
where
    T: Clone + Debug + Default + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign,
{
    fn default() -> Self {
        Self::new()
    }
}

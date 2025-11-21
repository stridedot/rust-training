use std::{ops::Deref, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;

use crate::resp::frame::RespFrame;

#[derive(Clone)]
pub struct Backend(Arc<BackendInner>);

pub struct BackendInner {
    pub map: DashMap<String, RespFrame>,
    pub hmap: DashMap<String, DashMap<String, RespFrame>>,
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&self, key: String, value: RespFrame) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) -> Result<()> {
        self.hmap.entry(key).or_default().insert(field, value);
        Ok(())
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        self.hmap
            .get(key)
            .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|v| v.clone())
    }
}

impl BackendInner {
    pub fn new() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self(Arc::new(BackendInner::new()))
    }
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

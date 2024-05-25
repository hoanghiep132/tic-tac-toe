use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{bail, Result};
use fnv::FnvHashMap;
use log::info;

use crate::conf::SwitchboardConfig;

#[derive(Debug)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub user_name: String,
}

#[derive(Debug)]
pub struct Switchboard {
    sessions: FnvHashMap<u64, Session>,
    cfg: SwitchboardConfig,
}

impl Switchboard {
    pub fn new(cfg: SwitchboardConfig) -> Self {
        Self {
            sessions: FnvHashMap::default(),
            cfg,
        }
    }

    pub fn insert_new_session(&mut self, session: Session) {
        let session_id = session.id;
        info!("Inserting session: {}", session_id);
        self.sessions.insert(
            session_id,
            session,
        );
    }

    pub fn sessions_count(&self) -> usize {
        self.sessions.len()
    }
}

#[derive(Debug)]
pub struct LockedSwitchboard(RwLock<Switchboard>);

impl LockedSwitchboard {
    pub fn new(cfg: SwitchboardConfig) -> Self {
        Self(RwLock::new(Switchboard::new(cfg)))
    }

    pub fn with_read_lock<F, R>(&self, callback: F) -> Result<R> where F: FnOnce(RwLockReadGuard<Switchboard>) -> Result<R>,
    {
        match self.0.read() {
            Ok(switchboard) => callback(switchboard),
            Err(_) => bail!("Failed to acquire switchboard read lock"),
        }
    }

    pub fn with_write_lock<F, R>(&self, callback: F) -> Result<R> where F: FnOnce(RwLockWriteGuard<Switchboard>) -> Result<R>,
    {
        match self.0.write() {
            Ok(switchboard) => callback(switchboard),
            Err(_) => bail!("Failed to acquire switchboard write lock"),
        }
    }
}
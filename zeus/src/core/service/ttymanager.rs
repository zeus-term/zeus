use log::info;
use nix::{
    fcntl::OFlag,
    pty::{grantpt, posix_openpt, ptsname_r, PtyMaster},
};
use std::collections::HashMap;

pub struct TtyManager {
    active_ttys: u32,
    pty_cache: HashMap<String, PtyMaster>,
}

impl Default for TtyManager {
    fn default() -> Self {
        TtyManager::new()
    }
}

impl TtyManager {
    pub fn new() -> TtyManager {
        TtyManager {
            active_ttys: 0,
            pty_cache: HashMap::new(),
        }
    }

    // pub fn create_pty(&mut self) -> Result<String, ()> {}
}

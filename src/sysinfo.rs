//! This file works with general system information, using the modules in sysinfo.

mod battery;
mod net;
mod os;

use self::net::NetworkInfo;

/// # System Info
///
/// Used to fetch general system information.
pub struct SystemInfo {
    network_info: NetworkInfo,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            network_info: NetworkInfo::new(),
        }
    }

    pub fn network_info(&mut self) -> &mut NetworkInfo {
        &mut self.network_info
    }
}

//! Network code for the sysinfo module.

use std::net::Ipv4Addr;

use crate::sysinfo::os::target;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Network {
    ssid: String,
}

impl Network {
    pub const fn new(ssid: String) -> Self {
        Self { ssid }
    }

    pub fn ssid(&self) -> &str {
        &self.ssid
    }
}

/// Networking information.
pub struct NetworkInfo {
    networks: Vec<Network>,
    public_ip: Option<Ipv4Addr>,
}

impl NetworkInfo {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            public_ip: None,
        }
    }

    pub fn networks(&self) -> &Vec<Network> {
        &self.networks
    }

    pub fn public_ip(&self) -> Ipv4Addr {
        return self.public_ip.unwrap_or(Ipv4Addr::new(127, 0, 0, 1));
    }

    pub fn get_public_ip(&mut self) -> &Self {
        
        self
    }

    pub fn scan_for_networks(&mut self) -> &Self {
        let networks = target::net::enum_networks();

        if let Some(ref networks) = networks {
            self.networks = networks.clone();
        }

        self
    }
}

//! Linux-specific code, group these into the modules they are used in.

#![cfg(target_os = "linux")]

pub mod net {
    use crate::sysinfo::net::Network;

    #[cfg(all(feature = "wireless"))]
    /// Scans for a list of nearby networks.
    pub fn enum_networks() -> Option<Vec<Network>> {
        use nmdbus::{accesspoint::AccessPoint, device_wireless::DeviceWireless, NetworkManager};
        use std::time::Duration;

        let mut ssid_list: Vec<String> = Vec::new();

        let conn = dbus::blocking::Connection::new_system();
        if let Ok(conn) = conn {
            let proxy = conn.with_proxy(
                "org.freedesktop.NetworkManager",
                "/org/freedesktop/NetworkManager",
                Duration::from_secs_f32(5.0),
            );

            if let Ok(dev) = proxy.get_all_devices() {
                for dev in dev {
                    let dev_proxy = conn.with_proxy(
                        "org.freedesktop.NetworkManager",
                        dev,
                        Duration::from_secs_f32(5.0),
                    );

                    if let Ok(aps) = dev_proxy.get_all_access_points() {
                        for ap in aps {
                            let ap_proxy = conn.with_proxy(
                                "org.freedesktop.NetworkManager",
                                ap,
                                Duration::from_secs_f32(5.0),
                            );

                            if let Ok(ssid) = ap_proxy.ssid() {
                                if let Ok(ssid) = String::from_utf8(ssid) {
                                    ssid_list.push(ssid);
                                }
                            }
                        }
                    }
                }
            }
        }

        if ssid_list.is_empty() {
            return None;
        }

        // Clear any duplicate entries.
        ssid_list.sort();
        ssid_list.dedup();

        let mut networks: Vec<Network> = Vec::new();
        for ssid in ssid_list {
            networks.push(Network::new(ssid));
        }

        Some(networks)
    }
}

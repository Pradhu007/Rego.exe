/// # System Info
///
/// Used to fetch general system information.
///
/// ## Contributor Note:
/// When adding to this struct, use modules and commit as such:
/// \[module\] short message.
///
/// See below in net for an example.
pub struct SystemInfo {
    network_info: net::NetworkInfo,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            network_info: net::NetworkInfo::new(),
        }
    }

    pub fn network_info(&self) -> &net::NetworkInfo {
        return &self.network_info;
    }
}

/// Contains network code for the sysinfo module.
pub mod net {

    /// Networking information.
    pub struct NetworkInfo {}
    impl NetworkInfo {
        pub fn new() -> Self {
            Self {}
        }

        pub fn networks(&self) {
            enum_networks();
        }
    }

    // Private/helper functions start here.

    #[cfg(target_os = "linux")]
    fn enum_networks() {
        use nmdbus::{device_wireless::DeviceWireless, NetworkManager};
        use std::time::Duration;

        let conn = dbus::blocking::Connection::new_system();
        if let Ok(conn) = conn {
            let proxy = conn.with_proxy(
                "org.freedesktop.NetworkManager",
                "/org/freedesktop/NetworkManager",
                Duration::from_secs_f32(5.0),
            );

            let dev = proxy.get_all_devices();

            match dev {
                Ok(dev) => {
                    println!("{}", dev.len());

                    for dev in dev {
                        let dev_proxy = conn.with_proxy(
                            "org.freedesktop.NetworkManager",
                            dev,
                            Duration::from_secs_f32(5.0),
                        );

                        dev_proxy.get_all_access_points();
                    }
                }
                Err(why) => {
                    println!("{}", why);
                }
            }
        } else {
        }
    }
}

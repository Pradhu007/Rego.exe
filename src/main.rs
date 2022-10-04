#![feature(doc_cfg)]
// #![allow(dead_code)]
#![deny(
    clippy::style,
    clippy::complexity,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::suspicious,
    clippy::perf,
    clippy::pedantic,
    clippy::correctness
)]

mod sysinfo;

fn main() {
    let mut sysinfo = sysinfo::SystemInfo::new();
    let networks = sysinfo.network_info().scan_for_networks().networks();
    for network in networks {
        println!("{}", network.ssid());
    }
}

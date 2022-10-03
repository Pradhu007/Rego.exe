#![feature(doc_cfg)]
#![allow(dead_code)]

mod sysinfo;

fn main() {
    let sysinfo = sysinfo::SystemInfo::new();
    sysinfo.network_info().networks();
}

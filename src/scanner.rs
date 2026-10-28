use crate::models::{Device, Status};
use crate::network::get_mac_address;
use std::net::Ipv4Addr;
use std::process::Command;
use tokio::task;

pub async fn scan_ip(ip: Ipv4Addr, wait: f64) -> Device {
    let ip_string = ip.to_string();
    let wait_string = wait.to_string();

    let is_available = task::spawn_blocking(move || {
        Command::new("ping")
            .args(["-c", "1", "-W", &wait_string, &ip_string])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false);

    Device {
        ip: ip.to_string(),
        mac: if is_available {
            get_mac_address(&ip.to_string())
        } else {
            None
        },
        status: if is_available {
            Status::Occupied
        } else {
            Status::NotFound
        },
    }
}

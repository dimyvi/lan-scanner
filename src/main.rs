mod models;
mod network;
mod scanner;

use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::stream::{self, StreamExt};
use models::{Device, Status};

#[derive(Parser)]
#[command(name = "lan-scanner")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(alias = "o")]
    Occupied {
        #[arg(short, long)]
        subnet: String,
        #[arg(short = 'w', long, default_value_t = 0.2)]
        wait: f64,
    },

    #[command(alias = "f")]
    Free {
        #[arg(short, long)]
        subnet: String,
        #[arg(short = 'w', long, default_value_t = 0.2)]
        wait: f64,
    },

    #[command(alias = "st")]
    Status {
        ip: String,
        #[arg(short = 'w', long, default_value_t = 0.2)]
        wait: f64,
    },
    #[command(alias = "r")]
    Range {
        #[arg(short = 'f', long)]
        from: String,
        #[arg(short = 't', long)]
        to: String,
        #[arg(short = 'w', long, default_value_t = 0.2)]
        wait: f64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Command::Status { ip, wait } = &cli.command {
        let address: std::net::Ipv4Addr = ip.parse()?;
        let device = scanner::scan_ip(address, *wait).await;

        if network::get_local_ipv4().contains(&address) {
            println!("Owner: LOCAL COMPUTER");
        }
        print_device_details(&device);
        return Ok(());
    }

    let (addresses, filter, wait) = match cli.command {
        Command::Occupied { subnet, wait } => (network::get_addresses(&subnet)?, Some(true), wait),
        Command::Free { subnet, wait } => (network::get_addresses(&subnet)?, Some(false), wait),
        Command::Range { from, to, wait } => (network::get_range(&from, &to)?, None, wait),
        Command::Status { .. } => unreachable!(),
    };

    let devices = stream::iter(addresses)
        .map(|ip| scanner::scan_ip(ip, wait))
        .buffer_unordered(64)
        .collect::<Vec<_>>()
        .await;

    let local_ips = network::get_local_ipv4();
    println!("{:<16} {:<16} {:<20} OWNER", "IP", "STATUS", "MAC");

    for device in devices.into_iter().filter(|device| match filter {
        Some(true) => matches!(device.status, Status::Occupied),
        Some(false) => matches!(device.status, Status::NotFound),
        None => true,
    }) {
        let status = match device.status {
            Status::Occupied => "occupied",
            Status::NotFound => "not responding",
        };
        println!(
            "{:<16} {:<16} {:<20} {}",
            device.ip,
            status,
            device.mac.unwrap_or_else(|| "-".to_string()),
            if local_ips.iter().any(|ip| ip.to_string() == device.ip) {
                "LOCAL"
            } else {
                ""
            }
        );
    }
    Ok(())
}

fn print_device_details(device: &Device) {
    println!("IP: {}", device.ip);
    match device.status {
        Status::Occupied => {
            println!("Status: occupied");
            println!("MAC: {}", device.mac.as_deref().unwrap_or("not found"));
        }
        Status::NotFound => {
            println!("Status: not responding");
            println!("MAC: not found");
        }
    }
}

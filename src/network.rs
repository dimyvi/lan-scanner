use anyhow::Result;
use ipnetwork::Ipv4Network;
use std::net::Ipv4Addr;
use std::process::Command;

pub fn get_addresses(subnet: &str) -> Result<Vec<Ipv4Addr>> {
    let network: Ipv4Network = subnet.parse()?;

    let addresses = network
        .iter()
        .filter(|ip| *ip != network.network() && *ip != network.broadcast())
        .collect();

    Ok(addresses)
}

pub fn get_range(from: &str, to: &str) -> Result<Vec<Ipv4Addr>> {
    let start: Ipv4Addr = from.parse()?;
    let end: Ipv4Addr = to.parse()?;
    let start_num = u32::from(start);
    let end_num = u32::from(end);

    if start_num > end_num {
        anyhow::bail!("the start IP address must be less than or equal to the end IP address");
    }

    Ok((start_num..=end_num).map(Ipv4Addr::from).collect())
}

pub fn get_local_ipv4() -> Vec<Ipv4Addr> {
    let output = match Command::new("ip")
        .args(["-o", "-4", "addr", "show"])
        .output()
    {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().nth(3))
        .filter_map(|address| address.split('/').next())
        .filter_map(|address| address.parse().ok())
        .collect()
}

pub fn get_mac_address(ip: &str) -> Option<String> {
    let output = Command::new("ip")
        .args(["neigh", "show", ip])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = stdout.split_whitespace().collect();

    let position = parts.iter().position(|part| *part == "lladdr")?;
    parts.get(position + 1).map(|mac| (*mac).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subnet_excludes_network_and_broadcast() {
        let addresses = get_addresses("192.168.1.0/30").unwrap();

        assert_eq!(
            addresses,
            vec![
                "192.168.1.1".parse::<Ipv4Addr>().unwrap(),
                "192.168.1.2".parse::<Ipv4Addr>().unwrap(),
            ]
        );
    }

    #[test]
    fn range_includes_both_endpoints() {
        let addresses = get_range("192.168.1.10", "192.168.1.12").unwrap();

        assert_eq!(addresses.len(), 3);
        assert_eq!(addresses[0], "192.168.1.10".parse::<Ipv4Addr>().unwrap());
        assert_eq!(addresses[2], "192.168.1.12".parse::<Ipv4Addr>().unwrap());
    }

    #[test]
    fn range_rejects_reversed_values() {
        assert!(get_range("192.168.1.20", "192.168.1.10").is_err());
    }
}

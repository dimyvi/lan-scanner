# lan-scanner

A simple LAN IP and MAC address scanner for Linux, written in Rust.

The tool can scan a subnet, scan a custom IP range, show occupied addresses with MAC addresses, and check one specific IP address.

## Features

- Scan a CIDR subnet.
- Show occupied IP addresses.
- Show non-responding IP addresses.
- Display MAC addresses when available.
- Check one specific IP address.
- Scan a custom inclusive IP range.
- Configure the ping timeout.
- Fast parallel scanning.

## Installation

Install from crates.io:

```bash
cargo install lan-scanner
```

Or run the project from source:

```bash
cargo run -- --help
```

## Usage

Show all available commands:

```bash
lan-scanner --help
```

### Show occupied IP addresses

```bash
lan-scanner o -s 192.168.1.0/24
```

The command also shows the MAC address when it is available.

The long form is:

```bash
lan-scanner occupied --subnet 192.168.1.0/24
```

### Show non-responding IP addresses

```bash
lan-scanner f -s 192.168.1.0/24
```

The long form is:

```bash
lan-scanner free --subnet 192.168.1.0/24
```

### Check one IP address

```bash
lan-scanner st 192.168.1.1
```

This shows whether the IP address responds and displays its MAC address when available.

The long form is:

```bash
lan-scanner status 192.168.1.1
```

### Scan a custom IP range

```bash
lan-scanner r -f 192.168.1.10 -t 192.168.1.30
```

The range includes both the start and end addresses.

The long form is:

```bash
lan-scanner range \
  --from 192.168.1.10 \
  --to 192.168.1.30
```

### Configure the timeout

The default timeout is `0.2` seconds. Use `-w` or `--wait` to change it:

```bash
lan-scanner o -s 192.168.1.0/24 -w 0.5
```

Higher values can improve detection reliability but make scanning slower.

## Development

Check the project:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

Build an optimized binary:

```bash
cargo build --release
```

The release binary is located at:

```text
target/release/lan-scanner
```

## Requirements

- Linux
- Rust and Cargo
- The `ping` and `ip` system commands

On Debian or Ubuntu, install them with:

```bash
sudo apt install iputils-ping iproute2
```

## Legal notice

Only scan networks and devices that you own or have explicit permission to scan.


# nanocl
Unlock all control of your network using nanocl

Setup and configure enterprice grade vpn with dns!
And automaticaly test, deploy and scale your services or applications.

Allow container and virtual machine management on multiple machine using swarm mode

## State

Currently refactoring everything in rust for better performance stability and scalability.
And i wanted to learn rust.
Also because it's fun right ?

## Compatibility

List of system compatible and tested
- Ubuntu 20.xx
- Ubuntu 22.xx

## Installation

### Required dev dependencies
- Ubuntu:
```sh
sudo apt install -y pkg-config libpq-dev libssl-dev
```

### Recommanded rust devtools
```sh
cargo install diesel_cli --no-default-features --features postgres
rustup component add llvm-tools-preview --toolchain stable-x86_64-unknown-linux-gnu
cargo install cargo-make
cargo install cargo-watch
cargo install cargo-nextest
cargo install cargo-llvm-cov
```

## Note

must read /sys/class/net and /proc/net to get network informations

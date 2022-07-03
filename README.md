<div align="center">
  <img
    src="https://avatars.githubusercontent.com/u/94208118?s=200&v=4"
  />
  <p><h1>nanocl</h1> </p>
  <p><strong>self-sufficient vms and containers manager</strong> </p>
</div>

Unlock control of your network using nanocl.

Setup and configure enterprice grade vpn with his own dns server and http proxy.
Allow vms and containers management on multiple machine.
Automaticaly test, deploy and scale your services or applications.

## State

Approching alpha-0.1 release

## Compatibility
List of system compatible and tested
- Ubuntu 20.xx
- Ubuntu 22.xx

## Installation

### From source
```sh
git clone https://github.com/leon3s/nanocl
cd nanocl
```

##### Required dev dependencies
docker and containerd must be installed aswell see [install docker engine on ubuntu](https://docs.docker.com/engine/install/ubuntu/)

We need pkg-config and libssl and lib postgresql installed on the system to be abble to compil rust code
- Ubuntu:
```sh
sudo apt install -y pkg-config libpq-dev libssl-dev
```

##### Recommanded rust devtools
```sh
cargo install diesel_cli --no-default-features --features postgres
rustup component add llvm-tools-preview --toolchain stable-x86_64-unknown-linux-gnu
cargo install cargo-make
cargo install cargo-watch
cargo install cargo-nextest
cargo install cargo-llvm-cov
```

#### RUN
if you want a custom dockerd and containerd running to not override the system default you can run pre_start.sh
```sh
sudo ./pre_start.sh
```

we also recommand to create a group nanocl and to add current user in it in order to access to /run/nanocl
```sh
sudo groupadd nanocl
sudo usermod -aG nanocl ${USER}
```
then you can start the daemon
```sh
cd nanocld
cargo run
```
He will automatically download required dependencies at boot time such as postgresql server with nginx and dnsmasq
you must have free port 80,433 and 53 if you want nanocl to be abble to service services depending on domain.

for the cli
```sh
cd nanocl/nanocli
cargo run --help
```

## Note

### Ubuntu
We may read /sys/class/net and /proc/net to get network informations but
for now user specify the ip address to bind to

You may start a custom docker service to in case of existing docker setups
that will be done at the after the installation using the pre_start script

```sh
sudo containerd --config fake_path/etc/nanocl/containerd.conf
sudo dockerd --config-file fake_path/etc/nanocl/dockerd.json
```

### Windows
You can develop under windows using wsl2
you just need to add dns entry inside WSL network interface to make it work properly

identify the ID of WSL interface
```powershell
Get-NetAdapter
```

you should see a line line this
```
vEthernet (WSL)           Hyper-V Virtual Ethernet Adapter          59 Up
```
then update interface dns
```powershell
Set-DnsClientServerAddress -InterfaceIndex 59 -ServerAddresses ("10.0.0.1","10.0.0.2")
```

## Generate doc .md
```sh
pandoc --from man --to markdown < nanocl-namespace.1 > nanocl-namespace.1.md
```

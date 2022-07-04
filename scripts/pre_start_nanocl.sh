#!/bin/sh
## name: pre_start_nanocl.sh
set -e -x

: ${bridge=nanocl}

# Set up bridge network:
if ! ip link show $bridge > /dev/null 2>&1
then
   sudo ip link add name $bridge type bridge
   sudo ip addr add ${net:-"142.0.0.1/24"} dev $bridge
   sudo ip link set dev $bridge up
fi

sudo mkdir -p /run/nanocl
sudo mkdir -p /var/lib/nanocl

sudo containerd --config /etc/nanocl/containerd.conf 2> /dev/null &
sudo dockerd --config-file /etc/nanocl/dockerd.json 2> /dev/null &

sudo chown root:nanocl -R /run/nanocl
sudo chmod 070 -R /run/nanocl

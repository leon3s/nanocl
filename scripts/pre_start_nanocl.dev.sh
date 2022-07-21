#!/bin/sh -i
## name: pre_start_nanocl.dev.sh
set -e -x

: ${bridge=nanocl0}

# Set up bridge network:
if ! ip link show $bridge > /dev/null 2>&1
then
   sudo ip link add name $bridge type bridge
   sudo ip addr add ${net:-"142.0.0.1/24"} dev $bridge
   sudo ip link set dev $bridge up
fi

sudo mkdir -p /run/nanocl
sudo mkdir -p /var/lib/nanocl

sudo containerd --config ./fake_path/etc/nanocl/containerd.conf 2> /dev/null &
sudo dockerd --config-file ./fake_path/etc/nanocl/dockerd.json 2> /dev/null &

sleep 4

sudo chmod 777 -R /run/nanocl
sudo chmod 777 -R /var/lib/nanocl

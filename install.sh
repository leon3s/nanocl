#!/bin/sh
## name: install.sh
set -e -x

: ${bridge=nanocl}

# Set up bridge network:
if ! ip link show $bridge > /dev/null 2>&1
then
   sudo ip link add name $bridge type bridge
   sudo ip addr add ${net:-"142.0.0.1/24"} dev $bridge
   sudo ip link set dev $bridge up
fi

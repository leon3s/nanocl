#!/bin/sh
## name: install_dev.sh
set -e -x

sudo groupapp nanocl
sudo mkdir -p /var/lib/nanocl
sudo mkdir -p /var/run/nanocl

#!/bin/sh
## name: install.sh
set -e -x

sudo groupapp nanocl
sudo cp -r ./var/lib/nanocl /var/lib/nanocl
sudo mkdir -p /var/run/nanocl

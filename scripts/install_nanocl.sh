#!/bin/sh
## name: install_nanocl.sh
set -e -x

sudo addgroup nanocl
sudo cp -r ./fake_path/var/lib/nanocl /var/lib/nanocl
sudo mkdir -p /var/run/nanocl

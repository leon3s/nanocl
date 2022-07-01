#!/bin/sh
## name: release.sh
set -e -x

cargo make release
mkdir -p target/nanocl_1.0-1_ia64
mkdir -p target/nanocl_1.0-1_ia64/usr/local/bin
mkdir -p target/nanocl_1.0-1_ia64/run/nanocl
mkdir -p target/nanocl_1.0-1_ia64/var/lib/nanocl
mkdir -p target/nanocl_1.0-1_ia64/var/lib/nanocl
mkdir -p target/nanocl_1.0-1_ia64/etc
mkdir -p target/nanocl_1.0-1_ia64/DEBIAN
cp -r fake_path/etc/nanocl target/nanocl_1.0-1_ia64/etc/nanocl
cp target/release/nanocli target/nanocl_1.0-1_ia64/usr/local/bin
cp target/release/nanocld target/nanocl_1.0-1_ia64/usr/local/bin

cat > target/nanocl_1.0-1_ia64/DEBIAN/control <<- EOM
Package: nanocl
Version: 0.1
Architecture: ia64
Maintainer: leone leone@next-hat.com
Description: A self-sufficient vms and containers manager
EOM

mkdir -p target/debian
dpkg-deb --build --root-owner-group target/nanocl_1.0-1_ia64 target/debian/nanocl_1.0-1_ia64.deb

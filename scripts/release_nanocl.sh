#!/bin/sh
## name: release_nanocl.sh
set -e -x

# variables
pkg_name="nanocl"
arch=`dpkg --print-architecture`
version=`cat ./nanocli/Cargo.toml | grep -m 1 "version = \"" | sed 's/[^0-9.]*\([0-9.]*\).*/\1/'`
release_path="../target/${pkg_name}_${version}_${arch}"

cd nanocli
# create directories structure for package
mkdir -p ${release_path}
mkdir -p ${release_path}/DEBIAN
mkdir -p ${release_path}/usr/local/bin
mkdir -p ${release_path}/usr/local/man/man1

# create and copy release binary
cargo make release
cp ../target/release/${pkg_name} ${release_path}/usr/local/bin

# generate man pages
mkdir -p ../target/man
cargo make man
pandoc --from man --to markdown < ../target/man/${pkg_name}.1 > ../man/${pkg_name}.1.md
gzip -f ../target/man/${pkg_name}.1
cp ../target/man/${pkg_name}.1.gz ${release_path}/usr/local/man/man1

# generate DEBIAN controll
cat > ${release_path}/DEBIAN/control <<- EOM
Package: ${pkg_name}
Version: ${version}
Architecture: ${arch}
Maintainer: next-hat team@next-hat.com
Description: A self-sufficient vms and containers manager
EOM

mkdir -p ../target/debian
dpkg-deb --build --root-owner-group ${release_path} ../target/debian/${pkg_name}_${version}_${arch}.deb

#!/bin/sh
## name: release_nanocld.sh
set -e -x

# variables
pkg_name="nanocld"
arch=`dpkg --print-architecture`
version=`cat ./nanocli/Cargo.toml | grep -m 1 "version = \"" | sed 's/[^0-9.]*\([0-9.]*\).*/\1/'`
release_path="../target/${pkg_name}_${version}_${arch}"

cd nanocld
# create directories structure for package
mkdir -p ${release_path}
mkdir -p ${release_path}/DEBIAN
mkdir -p ${release_path}/usr/local/bin
mkdir -p ${release_path}/usr/local/man/man1
mkdir -p ${release_path}/var/lib/nanocl
mkdir -p ${release_path}/etc

# create and copy release binary
cargo make release
cp ../target/release/${pkg_name} ${release_path}/usr/local/bin
# copy config files
cp -r ../fake_path/etc/nanocl ${release_path}/etc
cp -r ../fake_path/etc/systemd ${release_path}/etc
cp -r ../fake_path/etc/init.d ${release_path}/etc
cp -r ../fake_path/var/lib/nanocl/dnsmasq ${release_path}/var/lib/nanocl
cp -r ../fake_path/var/lib/nanocl/nginx ${release_path}/var/lib/nanocl
cp ../scripts/pre_start_nanocl.sh ${release_path}/var/lib/nanocl/prestart.sh

# TODO generate man pages
# mkdir -p ../target/man
# cargo make man
# pandoc --from man --to markdown < ../target/man/${pkg_name}.1 > ../man/${pkg_name}.1.md
# gzip -f ../target/man/${pkg_name}.1
# cp ../target/man/${pkg_name}.1.gz ${release_path}/usr/local/man/man1

# generate DEBIAN controll
cat > ${release_path}/DEBIAN/control <<- EOM
Package: ${pkg_name}
Version: ${version}
Architecture: ${arch}
Maintainer: next-hat team@next-hat.com
Description: A self-sufficient vms and containers manager
EOM

cat > ${release_path}/DEBIAN/postinst <<- EOM
EOM

chmod 775 ${release_path}/DEBIAN/postinst

cat > ${release_path}/DEBIAN/postrm <<- EOM
EOM

chmod 775 ${release_path}/DEBIAN/postrm

mkdir -p ../target/debian
dpkg-deb --build --root-owner-group ${release_path} ../target/debian/${pkg_name}_${version}_${arch}.deb

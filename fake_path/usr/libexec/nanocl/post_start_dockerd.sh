#!/bin/sh
## Ensure that group nanocl can read and write file after docker have been started

directories=""
directories="${directories} /var/lib/nanocl/ipsec"
directories="${directories} /var/lib/nanocl/nginx"
directories="${directories} /var/lib/nanocl/nginx/log"
directories="${directories} /var/lib/nanocl/nginx/sites-enabled"
directories="${directories} /var/lib/nanocl/dnsmasq"
directories="${directories} /var/lib/nanocl/dnsmasq/dnsmasq.d"

files=""
files="${files} /var/lib/nanocl/nginx/log/access.log"
files="${files} /var/lib/nanocl/dnsmasq/dnsmasq.d/dns_entry.conf"

fix_dir_perm () {
  chown -R :nanocl $1
  chmod 770 $1
}

fix_file_perm () {
  chown :nanocl $1
  chmod 660 $1
}

for directory in ${directories}; do
    fix_dir_perm ${directory}
done

for file in ${files}; do
    fix_file_perm ${file}
done

chmod 660 /var/lib/nanocl
chmod 770 /run/nanocl/docker.sock

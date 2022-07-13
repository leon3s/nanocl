NAME
====

nanocl - nanocli a self-sufficient vms and containers manager

SYNOPSIS
========

**nanocl** \[**-h**\|**\--help**\] \[**-V**\|**\--version**\]
\[**-H**\|**\--host**\] \<*subcommands*\>

DESCRIPTION
===========

nanocli a self-sufficient vms and containers manager

OPTIONS
=======

**-h**, **\--help**

:   Print help information

**-V**, **\--version**

:   Print version information

**-H**, **\--host**=*HOST* \[default: unix://run/nanocl/nanocl.sock\]

:   nanocld host

SUBCOMMANDS
===========

nanocl-docker(1)

:   alias to self-managed dockerd

nanocl-namespace(1)

:   manage namespaces

nanocl-cluster(1)

:   manage clusters

nanocl-cargo(1)

:   manage cargoes

nanocl-apply(1)

:   apply a configuration file

nanocl-revert(1)

:   revert a configuration file

nanocl-git-repository(1)

:   manage git repositories

nanocl-nginx-template(1)

:   

nanocl-cluster-network(1)

:   manage cluster networks

nanocl-container-image(1)

:   

nanocl-help(1)

:   Print this message or the help of the given subcommand(s)

VERSION
=======

v0.1.0

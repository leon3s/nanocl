# nanocl
Unlock all control of your network using nanocl

Setup and configure enterprice grade vpn, dns and automaticaly test, deploy and scale your applications.

## State

Currently refactoring everything in rust for better performance stability and scalability.

## Compatibility

List of system compatible and tested
- Ubuntu 20.xx
- Ubuntu 22.xx

## Installation

- Required dependencies
```sh
sudo apt install -y nginx nginx-extras dnsmasq docker-compose mongodb # For ubuntu
```

namespace docktron {
  network backend {
    cargo [
      mongodb
    ]

    cargo [
      api.docktron.com,
    ]
  }

  network frontend {
    cargo [
      docktron.com // main website
    ]
  }
}

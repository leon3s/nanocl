# nanocl
Unlock all control of your network using nanocl

Setup and configure enterprice grade vpn with dns!
And automaticaly test, deploy and scale your services or applications.

Allow container and virtual machine management on multiple machine using swarm mode

## State

Currently refactoring everything in rust for better performance stability and scalability.
And i wanted to learn rust.
Also because it's fun right ?

## Compatibility

List of system compatible and tested
- Ubuntu 20.xx
- Ubuntu 22.xx

## Installation

- Required dependencies
```sh
sudo apt install -y nginx nginx-extras dnsmasq docker-compose # For ubuntu
```

## Note

must read /sys/class/net and /proc/net to get network informations

## Shema of possible configuration

This is just for myself in order to plan what im doing

git_repository {
  required_services: [
    cargo mongodb
  ],
  hooks: {
    hook on_pull_request: {
      create_test_img,
      run_test_img,
      test_report,
      if success {
        create_deploy_img,
        run_img,
        wait_for_service,
        add_or_update_domain_name,
        restart_nginx,
      }
    }
    hook on_push {
      create_test_img,
      run_test_img,
      test_report,
      if success {
        create_deploy_img,
        run_img,
        wait_for_service,
        add_or_update_domain_name,
        restart_nginx,
      }
    }
  }
}

namespace docktron {
  virtual_machine ci {
    cluster test {
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
  }

  cluster development {
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
  cluster staging {
    network backend {
      cargo docktron_backend {
        env: [],
      }
    }

    network frontend {
      cargo [
        docktron.com // main website
      ]
    }
  }
}

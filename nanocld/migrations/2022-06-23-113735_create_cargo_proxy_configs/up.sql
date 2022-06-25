-- Your SQL goes here
CREATE TABLE "cargo_proxy_configs" (
  "cargo_key" VARCHAR NOT NULL UNIQUE PRIMARY KEY references cargos("key"),
  "domain_name" VARCHAR NOT NULL,
  "host_ip" VARCHAR NOT NULL
)

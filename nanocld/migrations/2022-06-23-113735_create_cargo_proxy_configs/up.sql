-- Your SQL goes here
CREATE TABLE "cargo_proxy_configs" (
  "cargo_key" VARCHAR NOT NULL UNIQUE PRIMARY KEY references cargoes("key"),
  "domain_name" VARCHAR NOT NULL,
  "host_ip" VARCHAR NOT NULL,
  "target_port" SERIAL NOT NULL
)

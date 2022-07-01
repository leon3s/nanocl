-- Your SQL goes here
CREATE TABLE "cluster_cargoes" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "cargo_key" VARCHAR NOT NULL references cargoes("key"),
  "cluster_key" VARCHAR NOT NULL references clusters("key"),
  "network_key" VARCHAR NOT NULL references cluster_networks("key")
);

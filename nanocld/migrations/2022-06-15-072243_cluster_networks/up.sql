-- Your SQL goes here
CREATE TABLE "cluster_networks" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "docker_network_id" VARCHAR NOT NULL UNIQUE,
  "cluster_key" VARCHAR NOT NULL references clusters("key")
);

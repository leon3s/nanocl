-- Your SQL goes here
CREATE TABLE "cluster_networks" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR(100) NOT NULL UNIQUE,
  "docker_network_id" VARCHAR NOT NULL UNIQUE,
  "cluster_id" UUID NOT NULL
);

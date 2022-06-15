-- Your SQL goes here
CREATE TABLE "cluster_networks" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR(100) NOT NULL UNIQUE,
  "cluster_id" UUID NOT NULL
);

-- Your SQL goes here
CREATE TABLE "cluster_variables" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "cluster_key" VARCHAR NOT NULL,
  "name" VARCHAR NOT NULL,
  "value" VARCHAR NOT NULL
);

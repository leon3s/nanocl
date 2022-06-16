-- Your SQL goes here
CREATE TABLE "clusters" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "namespace" VARCHAR NOT NULL
);

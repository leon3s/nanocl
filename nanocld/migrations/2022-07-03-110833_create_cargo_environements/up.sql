-- Your SQL goes here
CREATE TABLE "cargo_environnements" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "cargo_key" VARCHAR NOT NULL,
  "name" VARCHAR NOT NULL,
  "value" VARCHAR NOT NULL
);

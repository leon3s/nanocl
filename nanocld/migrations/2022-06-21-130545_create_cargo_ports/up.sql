-- Your SQL goes here
CREATE TABLE "cargo_ports" (
  "key" VARCHAR NOT NULL PRIMARY KEY,
  "cargo_key" VARCHAR NOT NULL,
  "from" SERIAL,
  "to" SERIAL
);

-- Your SQL goes here
create table "cargos" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "image_name" VARCHAR NOT NULL,
  "network_name" VARCHAR NOT NULL,
  "repository_name" VARCHAR NOT NULL,
  "namespace_name" VARCHAR NOT NULL references namespaces("name")
);

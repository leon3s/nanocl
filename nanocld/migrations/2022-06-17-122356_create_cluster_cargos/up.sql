-- Your SQL goes here
create table "cluster_cargos" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "cluster_key" VARCHAR NOT NULL references clusters("key")
);

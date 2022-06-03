-- Your SQL goes here
CREATE TABLE "clusters" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR(100) NOT NULL UNIQUE,
  "namespace" VARCHAR(100) NOT NULL
);

-- Your SQL goes here
CREATE TABLE "cargos" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "namespace" VARCHAR(100) NOT NULL,
  "name" VARCHAR(100) NOT NULL UNIQUE
);

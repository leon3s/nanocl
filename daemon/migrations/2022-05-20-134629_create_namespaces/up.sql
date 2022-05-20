-- Your SQL goes here
CREATE TABLE "namespaces" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR(100) NOT NULL UNIQUE
);

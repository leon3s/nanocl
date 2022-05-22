-- Your SQL goes here
CREATE TABLE "git_repositories" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "namespace" VARCHAR(100) NOT NULL,
  "uname": VARCHAR(100) NOT NULL UNIQUE
  "name" VARCHAR(100) NOT NULL,
  "url": VARCHAR(100) NOT NULL UNIQUE,
  "token": VARCHAR(100) NOT NULL
);

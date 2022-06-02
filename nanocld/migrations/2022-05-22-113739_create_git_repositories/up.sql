-- Your SQL goes here
CREATE TABLE "git_repositories" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "namespace" VARCHAR(100) NOT NULL,
  "name" VARCHAR(100) NOT NULL UNIQUE,
  "url" VARCHAR(100) NOT NULL,
  "token" VARCHAR(100) NOT NULL
);

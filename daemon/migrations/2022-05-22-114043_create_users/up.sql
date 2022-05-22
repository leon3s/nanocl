-- Your SQL goes here
CREATE TABLE "git_repositories" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR(100) NOT NULL UNIQUE,
  "passwd": VARCHAR(100) NOT NULL,
);

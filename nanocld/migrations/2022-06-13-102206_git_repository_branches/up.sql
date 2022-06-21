-- Your SQL goes here
CREATE TABLE "git_repository_branches" (
  "name" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "repository_name" VARCHAR NOT NULL
);

-- Your SQL goes here
CREATE TABLE "git_repository_branches" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "last_commit_sha" VARCHAR NOT NULL,
  "repository_name" VARCHAR NOT NULL
);

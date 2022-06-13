-- Your SQL goes here
CREATE TABLE "git_repository_branches" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR(100) NOT NULL,
  "repository_id" UUID NOT NULL
);

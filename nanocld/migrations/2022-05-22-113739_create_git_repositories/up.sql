-- Your SQL goes here
CREATE TYPE "git_repository_source_type" AS ENUM ('github', 'gitlab', 'local');

CREATE TABLE "git_repositories" (
  "name" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "url" VARCHAR NOT NULL,
  "default_branch" VARCHAR NOT NULL,
  "source" git_repository_source_type NOT NUll
);

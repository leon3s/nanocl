-- Your SQL goes here
CREATE TYPE "git_repository_source_type" AS ENUM ('github', 'gitlab', 'local');

CREATE TABLE "git_repositories" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "namespace" VARCHAR(100) NOT NULL,
  "name" VARCHAR(100) NOT NULL,
  "owner" VARCHAR(100) NOT NULL,
  "token" VARCHAR(100) NOT NULL,
  "source" git_repository_source_type NOT NUll
);

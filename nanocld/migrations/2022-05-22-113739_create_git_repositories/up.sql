-- Your SQL goes here
CREATE TYPE "git_repository_source_type" AS ENUM ('github', 'gitlab', 'local');

CREATE TABLE "git_repositories" (
  "id" UUID NOT NULL UNIQUE PRIMARY KEY,
  "name" VARCHAR(100) NOT NULL UNIQUE,
  "url" VARCHAR(200) NOT NULL,
  "token" VARCHAR NULL,
  "source" git_repository_source_type NOT NUll
);

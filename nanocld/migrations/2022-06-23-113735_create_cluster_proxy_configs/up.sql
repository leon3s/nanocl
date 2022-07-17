-- Your SQL goes here
CREATE TABLE "cluster_proxy_configs" (
  "cluster_key" VARCHAR NOT NULL UNIQUE PRIMARY KEY references clusters("key"),
  "template" TEXT[] NOT NULL,
  "target_port" SERIAL NOT NULL
)

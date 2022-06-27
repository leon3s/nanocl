-- Your SQL goes here
CREATE TABLE "nginx_templates" (
  "name" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "content" TEXT NOT NULL
);

INSERT INTO "nginx_templates" ("name", "content") VALUES ('nodejs-single', 'server {
  server_name {{domain_name}};
  listen {{host_ip}}:80;
  location / {
      proxy_set_header upgrade $http_upgrade;
      proxy_set_header connection "upgrade";
      proxy_http_version 1.1;
      proxy_set_header x-forwarded-for $proxy_add_x_forwarded_for;
      proxy_set_header host $host;
      proxy_pass http://{{target_ip}}:{{port}};
  }

  if ($host != {{domain}}) {
      return 502;
  }
}');

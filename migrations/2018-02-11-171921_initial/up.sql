-- Your SQL goes here

CREATE TABLE documentation (
  id SERIAL PRIMARY KEY,
  vcs VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  version VARCHAR NOT NULL
);

CREATE TABLE packages (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE releases (
  id SERIAL PRIMARY KEY,
  url VARCHAR,
  version VARCHAR NOT NULL
);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  vcs VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  version VARCHAR NOT NULL
);

CREATE TABLE package_releases (
  package_id SERIAL REFERENCES packages,
  release_id SERIAL REFERENCES releases
);

CREATE TABLE repositories (
  name VARCHAR PRIMARY KEY,
  documentation SERIAL REFERENCES documentation,
  release SERIAL REFERENCES releases,
  source SERIAL REFERENCES sources,
  status VARCHAR
);

CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  release_id SERIAL,
  name VARCHAR NOT NULL,
  value VARCHAR NOT NULL
);

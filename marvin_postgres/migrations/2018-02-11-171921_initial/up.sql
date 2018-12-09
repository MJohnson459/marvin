-- Your SQL goes here

CREATE TABLE packages (
  id              SERIAL PRIMARY KEY,
  name            VARCHAR NOT NULL,
  status          VARCHAR NOT NULL
);

CREATE TABLE subpackages (
  id              SERIAL PRIMARY KEY,
  package_id      INTEGER NOT NULL,
  name            VARCHAR NOT NULL
);

CREATE TABLE versions (
  id              SERIAL PRIMARY KEY,
  package_id      INTEGER NOT NULL,
  num             VARCHAR NOT NULL
);

CREATE TABLE documentation (
  id              SERIAL PRIMARY KEY,
  package_id      INTEGER NOT NULL,
  vcs             VARCHAR NOT NULL,
  url             VARCHAR NOT NULL,
  version         VARCHAR NOT NULL
);

CREATE TABLE source (
  id              SERIAL PRIMARY KEY,
  package_id      INTEGER NOT NULL,
  vcs             VARCHAR NOT NULL,
  url             VARCHAR NOT NULL,
  version         VARCHAR NOT NULL
);
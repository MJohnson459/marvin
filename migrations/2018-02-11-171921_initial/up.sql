-- Your SQL goes here
CREATE TYPE repository_status AS ENUM (
  'maintained',
  'developed'
);

CREATE TYPE supported_vcs AS ENUM (
  'git',
  'hg',
  'svn'
);

CREATE TABLE documentation (
  id SERIAL PRIMARY KEY,
  type supported_vcs NOT NULL,
  url VARCHAR NOT NULL,
  version VARCHAR NOT NULL
);

CREATE TABLE packages (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE releases (
  id SERIAL PRIMARY KEY,
  tags VARCHAR,
  url VARCHAR,
  version VARCHAR NOT NULL
);

CREATE TABLE sources (
  id SERIAL PRIMARY KEY,
  type supported_vcs NOT NULL,
  url VARCHAR NOT NULL,
  version VARCHAR NOT NULL
);

CREATE TABLE package_releases (
  package SERIAL REFERENCES packages,
  release SERIAL REFERENCES releases
);

CREATE TABLE repositories (
  name VARCHAR PRIMARY KEY,
  documentation SERIAL REFERENCES documentation,
  release SERIAL REFERENCES releases,
  source SERIAL REFERENCES sources,
  status repository_status
);

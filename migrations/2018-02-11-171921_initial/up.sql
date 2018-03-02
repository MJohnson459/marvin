-- Your SQL goes here

CREATE TABLE packages (
  id              SERIAL PRIMARY KEY,
  name            VARCHAR NOT NULL,
  updated_at      TIMESTAMP NOT NULL DEFAULT now(),
  created_at      TIMESTAMP NOT NULL DEFAULT now(),
  downloads       INTEGER NOT NULL DEFAULT 0,
  description     VARCHAR,
  homepage        VARCHAR,
  documentation   VARCHAR,
  textsearchable_index_col TSVECTOR,
  license         VARCHAR,
  repository      VARCHAR,
  max_upload_size INTEGER
);

CREATE TABLE package_owners (
  package_id      INTEGER NOT NULL,
  owner_id        INTEGER NOT NULL,
  created_at      TIMESTAMP NOT NULL DEFAULT now(),
  created_by      INTEGER,
  deleted         BOOL NOT NULL,
  updated_at      TIMESTAMP NOT NULL DEFAULT now()
);

ALTER TABLE package_owners ADD PRIMARY KEY (package_id, owner_id);

CREATE TABLE versions (
  id              SERIAL PRIMARY KEY,
  package_id      INTEGER,
  num             VARCHAR NOT NULL,
  updated_at      TIMESTAMP NOT NULL DEFAULT now(),
  created_at      TIMESTAMP NOT NULL DEFAULT now(),
  downloads       INTEGER NOT NULL,
  yanked          BOOL NOT NULL,
  license         VARCHAR
);

CREATE TABLE keywords (
  id              SERIAL PRIMARY KEY,
  keyword         VARCHAR NOT NULL,
  package_cnt     INTEGER NOT NULL,
  created_at      TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE users (
  id              SERIAL PRIMARY KEY,
  email           VARCHAR,
  name            VARCHAR
);


-- Your SQL goes here

CREATE TABLE packages (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL,
  downloads SERIAL NOT NULL,
  description VARCHAR,
  homepage VARCHAR,
  documentation VARCHAR,
  textsearchable_index_col TSVECTOR NOT NULL,
  license VARCHAR,
  repository VARCHAR,
  max_upload_size SERIAL
);

CREATE TABLE package_owners (
  package_id SERIAL NOT NULL,
  owner_id SERIAL NOT NULL,
  created_at TIMESTAMP NOT NULL,
  created_by SERIAL,
  deleted BOOL NOT NULL,
  updated_at TIMESTAMP NOT NULL
);

CREATE TABLE versions (
  id SERIAL PRIMARY KEY,
  package_id SERIAL,
  num VARCHAR NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL,
  downloads SERIAL NOT NULL,
  yanked BOOL NOT NULL,
  license VARCHAR
);

CREATE TABLE keywords (
  id SERIAL PRIMARY KEY,
  keyword VARCHAR NOT NULL,
  package_cnt SERIAL NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR,
  name VARCHAR
);


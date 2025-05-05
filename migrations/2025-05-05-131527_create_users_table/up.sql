-- Your SQL goes here
CREATE TABLE users (
  id uuid PRIMARY KEY,
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL
);
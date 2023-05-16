-- Your SQL goes here
CREATE TABLE personas (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  surname TEXT NOT NULL,
  birthday DATE NOT NULL
)
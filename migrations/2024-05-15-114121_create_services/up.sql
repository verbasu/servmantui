-- Your SQL goes here
CREATE TABLE services (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  back_path TEXT NOT NULL,
  front_path TEXT NOT NULL,
  back_env TEXT,
  front_env TEXT,
  icon_url TEXT,
  active BOOLEAN NOT NULL DEFAULT TRUE
);
CREATE UNIQUE INDEX idx_name_paths 
on services (name, back_path, front_path); 

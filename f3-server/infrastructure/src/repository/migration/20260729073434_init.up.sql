SET timezone TO 'Asia/Tokyo';

CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS hstore;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
  BEGIN
    new.updated_at := ''now'';
    return new;
  END;
' LANGUAGE 'plpgsql';

---------- role
CREATE TABLE IF NOT EXISTS roles (
  role_id VARCHAR(20) PRIMARY KEY
);

INSERT INTO roles (role_id)
VALUES 
  ('Admin'),
  ('Write'),
  ('Read');
---------- 

---------- operator
CREATE TABLE IF NOT EXISTS operators (
  operator_id TEXT PRIMARY KEY,
  password_hash TEXT NOT NULL,
  name VARCHAR(255) NOT NULL, 
  description VARCHAR(1024), 
  role VARCHAR(20) NOT NULL REFERENCES roles(role_id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

INSERT INTO operators (operator_id, password_hash, name, description, role)
VALUES (
  'administrator',
  crypt('password', gen_salt('bf')),
  'administrator',
  '全体管理者',
  'Admin'
);

CREATE TRIGGER operators_updated_at_trigger
  BEFORE UPDATE ON operators FOR EACH ROW
  EXECUTE PROCEDURE set_updated_at();
----------

---------- session
CREATE TABLE sessions (
	session_id  VARCHAR(64) PRIMARY KEY DEFAULT encode(gen_random_bytes(32), 'hex'),
	operator_id TEXT NOT NULL REFERENCES operators(operator_id),
  expire_at TIMESTAMPTZ NOT NULL DEFAULT now() + interval '24 hours',
	attribute   HSTORE
);
----------


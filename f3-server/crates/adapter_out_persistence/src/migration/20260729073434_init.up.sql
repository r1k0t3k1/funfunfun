SET timezone TO 'Asia/Tokyo';

CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS hstore;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
  BEGIN
    new.updated_at := ''now'';
    return new;
  END;
' LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION increment_version() RETURNS trigger AS '
  BEGIN
    NEW.version := OLD.version + 1;
    return new;
  END;
' LANGUAGE 'plpgsql';

---------- role
CREATE TABLE IF NOT EXISTS roles (
  name VARCHAR(20) PRIMARY KEY
);

INSERT INTO roles (name)
VALUES 
  ('Admin'),
  ('Write'),
  ('Read');
---------- 

---------- operator
CREATE TABLE IF NOT EXISTS operators (
	id uuid DEFAULT uuidv7() PRIMARY KEY,
  password_hash TEXT NOT NULL,
  name TEXT NOT NULL UNIQUE,
  description VARCHAR(1024), 
  role VARCHAR(20) NOT NULL REFERENCES roles(name),
  is_enabled BOOLEAN NOT NULL DEFAULT FALSE,
  version BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

INSERT INTO operators (name, password_hash, description, role, is_enabled)
VALUES (
  'administrator',
  '$argon2id$v=19$m=19456,t=2,p=1$OgYHvYo/Il/LCzNJtRTnHQ$i+oS59Fue3lg+4xKHOw96qqGWV8uZ+1ZPglSMwLFwKA', -- password
  '全体管理者',
  'Admin',
  TRUE
),
(
  'reader',
  '$argon2id$v=19$m=19456,t=2,p=1$OgYHvYo/Il/LCzNJtRTnHQ$i+oS59Fue3lg+4xKHOw96qqGWV8uZ+1ZPglSMwLFwKA', -- password
  '読み取り権限アカウント',
  'Read',
  TRUE
),
(
  'writer',
  '$argon2id$v=19$m=19456,t=2,p=1$OgYHvYo/Il/LCzNJtRTnHQ$i+oS59Fue3lg+4xKHOw96qqGWV8uZ+1ZPglSMwLFwKA', -- password
  '書き込み権限アカウント',
  'Write',
  TRUE
);

CREATE TRIGGER trg_10_operators_updated_at_trigger
  BEFORE UPDATE ON operators FOR EACH ROW
  EXECUTE PROCEDURE set_updated_at();

CREATE TRIGGER trg_20_operators_updated_at_trigger
  BEFORE UPDATE ON operators FOR EACH ROW
  EXECUTE PROCEDURE increment_version();
----------

---------- session
CREATE TABLE sessions (
	id uuid DEFAULT uuidv7() PRIMARY KEY,
	operator_id uuid NOT NULL REFERENCES operators(id),
  expire_at TIMESTAMPTZ NOT NULL DEFAULT now() + interval '24 hours',
	attribute   HSTORE
);
----------

---------- listener
CREATE TABLE listeners (
	id uuid DEFAULT uuidv7() PRIMARY KEY,
  name TEXT NOT NULL,
  lhost TEXT NOT NULL,
  lport INTEGER NOT NULL CHECK (lport > 0 AND lport <= 65535),
  is_running BOOLEAN NOT NULL, 
  checkin_key BYTEA NOT NULL DEFAULT gen_random_bytes(32) CHECK (octet_length(checkin_key) = 32), -- キー長32byteの制約
  config TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
----------

---------- agents
CREATE TABLE agents (
	id uuid DEFAULT uuidv7() PRIMARY KEY,
  listener_id uuid NOT NULL REFERENCES listeners(id),
  shared_secret BYTEA NOT NULL CHECK (octet_length(shared_secret) = 32), -- キー長32byteの制約
  process_id INTEGER NOT NULL,
  thread_id INTEGER NOT NULL,
  arch TEXT NOT NULL,  -- x64 or x86
  is_admin BOOLEAN NOT NULL,
  process_name TEXT NOT NULL,
  os TEXT NOT NULL,
  domain_name TEXT NOT NULL,
  computer_name TEXT NOT NULL,
  user_name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
----------

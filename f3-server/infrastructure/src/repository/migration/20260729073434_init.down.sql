-- Add down migration script here
DROP TRIGGER IF EXISTS operators_updated_at_trigger ON operators;
DROP TABLE IF EXISTS operators;

DROP FUNCTION set_updated_at;

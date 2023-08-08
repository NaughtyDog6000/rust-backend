CREATE TABLE IF NOT EXISTS tokens
(
    id BIGSERIAL NOT NULL,
    user_id bigint NOT NULL,
    epoch_expiry_date bigint NOT NULL,
    token VARCHAR(255) NOT NULL,
    creation_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT tokens_pkey PRIMARY KEY (id),
    CONSTRAINT user_id FOREIGN KEY (user_id) REFERENCES public.users (id) 

);



-- this is taken from https://www.the-art-of-web.com/sql/trigger-delete-old/ (25/7/23)

-- automatically triggers a function to delete all rows (tokens) older than 30 days
CREATE FUNCTION tokens_delete_old_rows() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
  row_count int;
BEGIN
  DELETE FROM tokens WHERE creation_timestamp < CURRENT_TIMESTAMP - INTERVAL '30 days';
    IF found THEN
    GET DIAGNOSTICS row_count = ROW_COUNT;
    RAISE NOTICE 'DELETED % row(s) FROM tokens', row_count;
  END IF;
  RETURN NULL;
END;
$$;


CREATE TRIGGER tokens_trigger_delete_old_rows
    AFTER INSERT ON tokens
    EXECUTE PROCEDURE tokens_delete_old_rows();
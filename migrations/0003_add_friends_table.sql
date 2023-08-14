CREATE TABLE IF NOT EXISTS friends
(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    sender_id bigint NOT NULL,
    receiver_id bigint NOT NULL,
    creation_timestamp TIMESTAMP NOT NULL,
    acceptance_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT sender_id FOREIGN KEY (sender_id) REFERENCES public.users (id),
    CONSTRAINT receiver_id FOREIGN KEY (receiver_id) REFERENCES public.users (id) 

);

CREATE TABLE IF NOT EXISTS friend_requests
(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    sender_id bigint NOT NULL,
    receiver_id bigint NOT NULL,
    creation_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT sender_id FOREIGN KEY (sender_id) REFERENCES public.users (id),
    CONSTRAINT receiver_id FOREIGN KEY (receiver_id) REFERENCES public.users (id) 

);


-- this is taken from https://www.the-art-of-web.com/sql/trigger-delete-old/ (25/7/23)

-- automatically triggers a function to delete all rows (tokens) older than 30 days
CREATE FUNCTION friend_requests_delete_old_rows() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
  row_count int;
BEGIN
  DELETE FROM friend_requests WHERE creation_timestamp < CURRENT_TIMESTAMP - INTERVAL '30 days';
    IF found THEN
    GET DIAGNOSTICS row_count = ROW_COUNT;
    RAISE NOTICE 'DELETED % row(s) FROM tokens', row_count;
  END IF;
  RETURN NULL;
END;
$$;


CREATE TRIGGER friend_requests_trigger_delete_old_rows
    AFTER INSERT ON tokens
    EXECUTE PROCEDURE friend_requests_delete_old_rows();
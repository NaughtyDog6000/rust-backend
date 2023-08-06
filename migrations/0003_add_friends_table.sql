CREATE TABLE IF NOT EXISTS friends
(
    id BIGSERIAL NOT NULL PRIMARY KEY,
    sender_id bigint NOT NULL,
    reciever_id bigint NOT NULL,
    creation_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    accepted BOOLEAN NOT NULL DEFAULT FALSE,
    declined BOOLEAN NOT NULL DEFAULT FALSE,
    response_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,



    CONSTRAINT sender_id FOREIGN KEY (id) REFERENCES public.users (id),
    CONSTRAINT reciever_id FOREIGN KEY (id) REFERENCES public.users (id) 

);
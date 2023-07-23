CREATE TABLE IF NOT EXISTS scores (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    epoch_upload_time BIGINT NOT NULL,
    score int NOT NULL,
    game_mode varchar(30),
    
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);
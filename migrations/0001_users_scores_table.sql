CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    username VARCHAR(30) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    epoch_signup_time BIGINT NOT NULL,
    date_of_birth DATE
);

CREATE TABLE IF NOT EXISTS scores (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    epoch_upload_time BIGINT NOT NULL,
    score int NOT NULL,
    game_mode varchar(30),
    epoch_game_start_time BIGINT NOT NULL,
    epoch_game_end_time BIGINT NOT NULL,
    
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id)
);
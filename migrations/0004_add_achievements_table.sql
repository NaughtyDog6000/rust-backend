CREATE TABLE IF NOT EXISTS achievements
(
  id BIGSERIAL NOT NULL PRIMARY KEY,
  "name" VARCHAR(63) NOT NULL,
  "description" VARCHAR(255),
  creation_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  tier SMALLINT NOT NULL DEFAULT 0,
  image_location VARCHAR(100),
  unlock_code VARCHAR(30)
);

CREATE TABLE IF NOT EXISTS achievement_unlocks
(
  id BIGSERIAL NOT NULL PRIMARY KEY,
  user_id bigint NOT NULL,
  achievement_id bigint NOT NULL,
  unlock_timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,


  CONSTRAINT user_id FOREIGN KEY (user_id) REFERENCES public.users (id),
  CONSTRAINT achievement_id FOREIGN KEY (achievement_id) REFERENCES public.achievements (id)
);
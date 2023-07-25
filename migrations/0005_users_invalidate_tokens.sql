ALTER TABLE users
-- ADD COLUMN IF NOT EXISTS epoch_invalidate_tokens BIGINT,
ADD COLUMN IF NOT EXISTS date_of_birth DATE;
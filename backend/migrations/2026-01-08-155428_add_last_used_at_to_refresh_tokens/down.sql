-- Remove last_used_at column from refresh_tokens table
ALTER TABLE refresh_tokens DROP COLUMN last_used_at;

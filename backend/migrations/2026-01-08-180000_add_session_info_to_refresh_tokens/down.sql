-- Remove user_agent column from refresh_tokens table

ALTER TABLE refresh_tokens DROP COLUMN IF EXISTS user_agent;

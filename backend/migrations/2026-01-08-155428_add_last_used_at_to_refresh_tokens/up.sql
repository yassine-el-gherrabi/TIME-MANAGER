-- Add last_used_at column to refresh_tokens table
-- This tracks when the token was last used for session activity display
ALTER TABLE refresh_tokens
ADD COLUMN last_used_at TIMESTAMP NOT NULL DEFAULT NOW();

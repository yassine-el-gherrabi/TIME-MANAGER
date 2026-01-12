-- Add user_agent column to refresh_tokens table
-- This field stores session information for the "Active Sessions" feature

ALTER TABLE refresh_tokens ADD COLUMN user_agent VARCHAR(512);

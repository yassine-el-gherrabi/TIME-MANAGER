-- Add password metadata fields to users table
ALTER TABLE users
    ADD COLUMN password_changed_at TIMESTAMP,
    ADD COLUMN password_expires_at TIMESTAMP,
    ADD COLUMN failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN locked_until TIMESTAMP;

-- Create index for locked_until to efficiently find locked accounts
CREATE INDEX idx_users_locked_until ON users(locked_until);

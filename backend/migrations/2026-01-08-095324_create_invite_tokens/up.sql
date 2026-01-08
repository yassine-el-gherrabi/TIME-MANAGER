-- Create invite_tokens table for user invitation workflow
CREATE TABLE invite_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(64) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for fast token lookup
CREATE INDEX idx_invite_tokens_token_hash ON invite_tokens(token_hash);

-- Index for user lookup
CREATE INDEX idx_invite_tokens_user_id ON invite_tokens(user_id);

-- Index for finding valid (non-expired, non-used) tokens
CREATE INDEX idx_invite_tokens_valid ON invite_tokens(user_id, expires_at) WHERE used_at IS NULL;

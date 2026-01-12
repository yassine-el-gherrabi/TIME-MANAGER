-- Add soft delete column to users table
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMPTZ NULL;

-- Index for efficient active user queries (most common case)
CREATE INDEX idx_users_active ON users(organization_id) WHERE deleted_at IS NULL;

-- Index for listing deleted users (admin functionality)
CREATE INDEX idx_users_deleted ON users(organization_id, deleted_at) WHERE deleted_at IS NOT NULL;

COMMENT ON COLUMN users.deleted_at IS 'Soft delete timestamp. NULL = active, NOT NULL = deleted';

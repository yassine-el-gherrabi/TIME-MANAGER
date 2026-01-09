-- Remove soft delete indexes
DROP INDEX IF EXISTS idx_users_deleted;
DROP INDEX IF EXISTS idx_users_active;

-- Remove soft delete column
ALTER TABLE users DROP COLUMN deleted_at;

-- Note: PostgreSQL does not support removing enum values directly.
-- This migration cannot be fully reversed without recreating the enum type.
-- In a real scenario, you would need to:
-- 1. Create a new enum type without super_admin
-- 2. Update all tables using the enum
-- 3. Drop the old enum and rename the new one
-- For development purposes, we'll leave this as a warning.

-- WARNING: This down migration is a no-op.
-- To fully reverse, you must recreate the database or manually fix the enum.
SELECT 1;

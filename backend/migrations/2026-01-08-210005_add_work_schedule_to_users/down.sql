-- Remove work_schedule_id from users table
DROP INDEX IF EXISTS idx_users_work_schedule;
ALTER TABLE users DROP COLUMN IF EXISTS work_schedule_id;

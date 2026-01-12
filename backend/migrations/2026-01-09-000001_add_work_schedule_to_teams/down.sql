-- Remove work_schedule_id from teams table
DROP INDEX IF EXISTS idx_teams_work_schedule;
ALTER TABLE teams DROP COLUMN IF EXISTS work_schedule_id;

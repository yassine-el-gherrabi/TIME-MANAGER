-- Drop clock_entries table
DROP INDEX IF EXISTS idx_clock_entries_user_open;
DROP INDEX IF EXISTS idx_clock_entries_status;
DROP INDEX IF EXISTS idx_clock_entries_clock_in;
DROP INDEX IF EXISTS idx_clock_entries_user;
DROP INDEX IF EXISTS idx_clock_entries_organization;
DROP TABLE IF EXISTS clock_entries;

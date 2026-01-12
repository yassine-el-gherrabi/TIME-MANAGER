-- Drop triggers
DROP TRIGGER IF EXISTS set_break_policies_updated_at ON break_policies;

-- Drop indexes
DROP INDEX IF EXISTS idx_break_entries_user_date;
DROP INDEX IF EXISTS idx_break_entries_clock_entry_id;
DROP INDEX IF EXISTS idx_break_entries_user_id;
DROP INDEX IF EXISTS idx_break_entries_org_id;
DROP INDEX IF EXISTS idx_break_windows_day;
DROP INDEX IF EXISTS idx_break_windows_policy_id;
DROP INDEX IF EXISTS idx_break_policies_active;
DROP INDEX IF EXISTS idx_break_policies_user_id;
DROP INDEX IF EXISTS idx_break_policies_team_id;
DROP INDEX IF EXISTS idx_break_policies_org_id;

-- Drop tables (in reverse order of dependencies)
DROP TABLE IF EXISTS break_entries;
DROP TABLE IF EXISTS break_windows;
DROP TABLE IF EXISTS break_policies;

-- Drop enum
DROP TYPE IF EXISTS break_tracking_mode;

-- Add max_daily_clock_events column to clock_restrictions table
-- NULL = unlimited, otherwise max number of clock entries per day
ALTER TABLE clock_restrictions
ADD COLUMN max_daily_clock_events INTEGER DEFAULT NULL;

COMMENT ON COLUMN clock_restrictions.max_daily_clock_events IS
'Maximum clock entries (in+out pairs) allowed per day. NULL means unlimited.';

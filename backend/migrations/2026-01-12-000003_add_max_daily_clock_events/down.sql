-- Remove max_daily_clock_events column from clock_restrictions table
ALTER TABLE clock_restrictions
DROP COLUMN max_daily_clock_events;

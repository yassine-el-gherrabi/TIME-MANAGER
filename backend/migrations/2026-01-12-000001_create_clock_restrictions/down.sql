-- Drop indexes
DROP INDEX IF EXISTS idx_clock_override_requests_pending;
DROP INDEX IF EXISTS idx_clock_override_requests_status;
DROP INDEX IF EXISTS idx_clock_override_requests_user;
DROP INDEX IF EXISTS idx_clock_override_requests_org;
DROP INDEX IF EXISTS idx_clock_restrictions_active;
DROP INDEX IF EXISTS idx_clock_restrictions_user;
DROP INDEX IF EXISTS idx_clock_restrictions_team;
DROP INDEX IF EXISTS idx_clock_restrictions_org;

-- Drop tables
DROP TABLE IF EXISTS clock_override_requests;
DROP TABLE IF EXISTS clock_restrictions;

-- Drop enums
DROP TYPE IF EXISTS clock_override_status;
DROP TYPE IF EXISTS clock_restriction_mode;

-- Drop absences table
DROP INDEX IF EXISTS idx_absences_type;
DROP INDEX IF EXISTS idx_absences_dates;
DROP INDEX IF EXISTS idx_absences_org_status;
DROP INDEX IF EXISTS idx_absences_user;
DROP TABLE IF EXISTS absences;

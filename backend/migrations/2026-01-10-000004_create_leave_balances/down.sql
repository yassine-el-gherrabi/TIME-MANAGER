-- Drop leave balances table
DROP INDEX IF EXISTS idx_leave_balances_type;
DROP INDEX IF EXISTS idx_leave_balances_org_year;
DROP INDEX IF EXISTS idx_leave_balances_user;
DROP TABLE IF EXISTS leave_balances;

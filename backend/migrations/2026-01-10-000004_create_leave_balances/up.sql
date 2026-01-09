-- Create leave balances table
CREATE TABLE leave_balances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    absence_type_id UUID NOT NULL REFERENCES absence_types(id) ON DELETE CASCADE,
    year INT NOT NULL,
    initial_balance DECIMAL(4,1) NOT NULL DEFAULT 0,
    used DECIMAL(4,1) NOT NULL DEFAULT 0,
    adjustment DECIMAL(4,1) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, absence_type_id, year),
    CONSTRAINT chk_leave_balances_year CHECK (year >= 2000 AND year <= 2100),
    CONSTRAINT chk_leave_balances_initial CHECK (initial_balance >= 0),
    CONSTRAINT chk_leave_balances_used CHECK (used >= 0)
);

-- Indexes for common queries
CREATE INDEX idx_leave_balances_user ON leave_balances(user_id);
CREATE INDEX idx_leave_balances_org_year ON leave_balances(organization_id, year);
CREATE INDEX idx_leave_balances_type ON leave_balances(absence_type_id);

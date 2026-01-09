-- Create closed_days table (company holidays/closures)
CREATE TABLE closed_days (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    date DATE NOT NULL,
    is_recurring BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for efficient date range queries
CREATE INDEX idx_closed_days_org_date ON closed_days(organization_id, date);

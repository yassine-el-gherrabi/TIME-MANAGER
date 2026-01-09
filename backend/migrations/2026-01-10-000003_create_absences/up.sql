-- Create absences table
CREATE TABLE absences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type_id UUID NOT NULL REFERENCES absence_types(id) ON DELETE RESTRICT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    days_count DECIMAL(4,1) NOT NULL,
    status absence_status NOT NULL DEFAULT 'pending',
    reason TEXT,
    rejection_reason TEXT,
    approved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_absences_dates CHECK (end_date >= start_date),
    CONSTRAINT chk_absences_days_count CHECK (days_count > 0)
);

-- Indexes for common queries
CREATE INDEX idx_absences_user ON absences(user_id);
CREATE INDEX idx_absences_org_status ON absences(organization_id, status);
CREATE INDEX idx_absences_dates ON absences(start_date, end_date);
CREATE INDEX idx_absences_type ON absences(type_id);

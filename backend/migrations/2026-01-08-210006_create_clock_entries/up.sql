-- Create clock_entries table
CREATE TABLE clock_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    clock_in TIMESTAMPTZ NOT NULL,
    clock_out TIMESTAMPTZ,
    status clock_entry_status NOT NULL DEFAULT 'pending',
    approved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    approved_at TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clock_entries_organization ON clock_entries(organization_id);
CREATE INDEX idx_clock_entries_user ON clock_entries(user_id);
CREATE INDEX idx_clock_entries_clock_in ON clock_entries(clock_in);
CREATE INDEX idx_clock_entries_status ON clock_entries(status);
CREATE INDEX idx_clock_entries_user_open ON clock_entries(user_id) WHERE clock_out IS NULL;

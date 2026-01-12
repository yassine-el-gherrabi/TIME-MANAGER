-- Create absence types table
CREATE TABLE absence_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(20) NOT NULL,
    color VARCHAR(7) DEFAULT '#3B82F6',
    requires_approval BOOLEAN NOT NULL DEFAULT true,
    affects_balance BOOLEAN NOT NULL DEFAULT true,
    is_paid BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organization_id, code)
);

-- Index for faster lookups by organization
CREATE INDEX idx_absence_types_organization ON absence_types(organization_id);

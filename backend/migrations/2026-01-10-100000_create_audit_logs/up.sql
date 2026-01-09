-- Create audit_action enum type
CREATE TYPE audit_action AS ENUM ('create', 'update', 'delete');

-- Create audit_logs table (append-only)
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action audit_action NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,
    old_values JSONB,
    new_values JSONB,
    ip_address VARCHAR(45),
    user_agent VARCHAR(512),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for efficient querying
CREATE INDEX idx_audit_logs_organization ON audit_logs(organization_id);
CREATE INDEX idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_entity_type ON audit_logs(entity_type);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_logs_org_entity_created ON audit_logs(organization_id, entity_type, created_at DESC);

COMMENT ON TABLE audit_logs IS 'Append-only audit trail. Records should never be modified or deleted.';

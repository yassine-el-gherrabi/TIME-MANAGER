-- Clock Restriction Mode Enum
-- strict: No override possible, must be within time window
-- flexible: Override with justification (auto-approved or pending manager approval)
-- unrestricted: No time restrictions enforced
CREATE TYPE clock_restriction_mode AS ENUM ('strict', 'flexible', 'unrestricted');

-- Clock Restrictions Table
-- Defines when users can clock in/out
-- Cascade: User config > Team config > Org config
CREATE TABLE clock_restrictions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    mode clock_restriction_mode NOT NULL DEFAULT 'flexible',
    clock_in_earliest TIME,                   -- e.g., 07:00 (NULL = no restriction)
    clock_in_latest TIME,                     -- e.g., 10:00 (NULL = no restriction)
    clock_out_earliest TIME,                  -- e.g., 16:00 (NULL = no restriction)
    clock_out_latest TIME,                    -- e.g., 22:00 (NULL = no restriction)
    enforce_schedule BOOLEAN NOT NULL DEFAULT true, -- Must be a working day per schedule
    require_manager_approval BOOLEAN NOT NULL DEFAULT false, -- For flexible mode overrides
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- One restriction config per scope level (org, team, or user)
    CONSTRAINT clock_restrictions_unique_scope UNIQUE (organization_id, team_id, user_id),
    -- Ensure valid hierarchy: org-wide (both NULL), team-level (user NULL), or user-level
    CONSTRAINT clock_restrictions_valid_hierarchy CHECK (
        (team_id IS NULL AND user_id IS NULL) OR -- org-wide
        (team_id IS NOT NULL AND user_id IS NULL) OR -- team-level
        (user_id IS NOT NULL) -- user-level (team can be NULL or NOT NULL)
    )
);

-- Clock Override Requests Table
-- For flexible mode: users can request override with justification
CREATE TYPE clock_override_status AS ENUM ('pending', 'approved', 'rejected', 'auto_approved');

CREATE TABLE clock_override_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    clock_entry_id UUID REFERENCES clock_entries(id) ON DELETE SET NULL, -- Linked after action
    requested_action VARCHAR(10) NOT NULL CHECK (requested_action IN ('clock_in', 'clock_out')),
    requested_at TIMESTAMPTZ NOT NULL, -- When the user wanted to clock
    reason TEXT NOT NULL,
    status clock_override_status NOT NULL DEFAULT 'pending',
    reviewed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    reviewed_at TIMESTAMPTZ,
    review_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_clock_restrictions_org ON clock_restrictions(organization_id);
CREATE INDEX idx_clock_restrictions_team ON clock_restrictions(team_id) WHERE team_id IS NOT NULL;
CREATE INDEX idx_clock_restrictions_user ON clock_restrictions(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_clock_restrictions_active ON clock_restrictions(organization_id, is_active) WHERE is_active = true;

CREATE INDEX idx_clock_override_requests_org ON clock_override_requests(organization_id);
CREATE INDEX idx_clock_override_requests_user ON clock_override_requests(user_id);
CREATE INDEX idx_clock_override_requests_status ON clock_override_requests(status) WHERE status = 'pending';
CREATE INDEX idx_clock_override_requests_pending ON clock_override_requests(organization_id, status) WHERE status = 'pending';

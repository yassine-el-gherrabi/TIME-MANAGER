-- Create updated_at trigger function if it doesn't exist
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create break tracking mode enum
CREATE TYPE break_tracking_mode AS ENUM ('auto_deduct', 'explicit_tracking');

-- Break policies table
-- Defines break policy at Organization, Team, or User level
CREATE TABLE break_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    tracking_mode break_tracking_mode NOT NULL DEFAULT 'auto_deduct',
    notify_missing_break BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Unique constraint: One policy per scope level
    UNIQUE(organization_id, team_id, user_id)
);

-- Break windows table
-- Defines when breaks must/should be taken
CREATE TABLE break_windows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    break_policy_id UUID NOT NULL REFERENCES break_policies(id) ON DELETE CASCADE,
    day_of_week SMALLINT NOT NULL CHECK (day_of_week >= 0 AND day_of_week <= 6),
    window_start TIME NOT NULL,
    window_end TIME NOT NULL,
    min_duration_minutes INT NOT NULL CHECK (min_duration_minutes > 0),
    max_duration_minutes INT NOT NULL CHECK (max_duration_minutes >= min_duration_minutes),
    is_mandatory BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- One window per day per policy
    UNIQUE(break_policy_id, day_of_week),
    -- Ensure window_start < window_end
    CHECK (window_start < window_end)
);

-- Break entries table
-- Records actual breaks taken (for explicit_tracking mode)
CREATE TABLE break_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    clock_entry_id UUID NOT NULL REFERENCES clock_entries(id) ON DELETE CASCADE,
    break_start TIMESTAMPTZ NOT NULL,
    break_end TIMESTAMPTZ,
    duration_minutes INT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Ensure break_end > break_start when set
    CHECK (break_end IS NULL OR break_end > break_start)
);

-- Indexes for break_policies
CREATE INDEX idx_break_policies_org_id ON break_policies(organization_id);
CREATE INDEX idx_break_policies_team_id ON break_policies(team_id) WHERE team_id IS NOT NULL;
CREATE INDEX idx_break_policies_user_id ON break_policies(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_break_policies_active ON break_policies(organization_id, is_active);

-- Indexes for break_windows
CREATE INDEX idx_break_windows_policy_id ON break_windows(break_policy_id);
CREATE INDEX idx_break_windows_day ON break_windows(break_policy_id, day_of_week);

-- Indexes for break_entries
CREATE INDEX idx_break_entries_org_id ON break_entries(organization_id);
CREATE INDEX idx_break_entries_user_id ON break_entries(user_id);
CREATE INDEX idx_break_entries_clock_entry_id ON break_entries(clock_entry_id);
CREATE INDEX idx_break_entries_user_date ON break_entries(user_id, break_start);

-- Trigger to update break_policies.updated_at
CREATE TRIGGER set_break_policies_updated_at
    BEFORE UPDATE ON break_policies
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

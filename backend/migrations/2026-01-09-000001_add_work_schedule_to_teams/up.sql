-- Add work_schedule_id to teams table
ALTER TABLE teams
ADD COLUMN work_schedule_id UUID REFERENCES work_schedules(id) ON DELETE SET NULL;

-- Add index for performance
CREATE INDEX idx_teams_work_schedule ON teams(work_schedule_id);

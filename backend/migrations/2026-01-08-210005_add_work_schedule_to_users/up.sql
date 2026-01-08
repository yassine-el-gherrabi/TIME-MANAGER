-- Add work_schedule_id to users table
ALTER TABLE users ADD COLUMN work_schedule_id UUID REFERENCES work_schedules(id) ON DELETE SET NULL;

CREATE INDEX idx_users_work_schedule ON users(work_schedule_id);

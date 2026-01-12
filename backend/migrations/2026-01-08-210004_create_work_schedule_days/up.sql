-- Create work_schedule_days table
CREATE TABLE work_schedule_days (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_schedule_id UUID NOT NULL REFERENCES work_schedules(id) ON DELETE CASCADE,
    day_of_week SMALLINT NOT NULL CHECK (day_of_week BETWEEN 0 AND 6),
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    break_minutes INTEGER NOT NULL DEFAULT 0 CHECK (break_minutes >= 0),
    UNIQUE(work_schedule_id, day_of_week)
);

CREATE INDEX idx_work_schedule_days_schedule ON work_schedule_days(work_schedule_id);

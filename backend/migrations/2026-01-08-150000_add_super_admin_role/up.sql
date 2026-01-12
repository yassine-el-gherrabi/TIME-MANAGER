-- Add super_admin role to user_role enum
ALTER TYPE user_role ADD VALUE 'super_admin' BEFORE 'admin';

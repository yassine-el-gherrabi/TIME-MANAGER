-- Seed data for development and testing
-- Based on TIME MANAGER REFERENCE.md specification
--
-- ⚠️ DEVELOPMENT PASSWORD: "Password123!"
-- The Argon2id hash below was generated using the backend's PasswordService
-- with Argon2::default() parameters (m=19456, t=2, p=1)

-- ============================================
-- Demo Organization
-- ============================================
INSERT INTO organizations (id, name, slug, timezone)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'Demo Company',
    'demo',
    'Europe/Paris'
) ON CONFLICT (id) DO NOTHING;

-- ============================================
-- Users (4 roles hierarchy)
-- ============================================
-- Password: "Password123!" for all users
-- Hash generated with: cargo test test_generate_seed_hash -- --nocapture

-- Super Admin user
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000010',
    '00000000-0000-0000-0000-000000000001',
    'superadmin@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Super',
    'Admin',
    'super_admin'
) ON CONFLICT (id) DO NOTHING;

-- Admin user
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000011',
    '00000000-0000-0000-0000-000000000001',
    'admin@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Admin',
    'User',
    'admin'
) ON CONFLICT (id) DO NOTHING;

-- Manager user
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000012',
    '00000000-0000-0000-0000-000000000001',
    'manager@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Manager',
    'User',
    'manager'
) ON CONFLICT (id) DO NOTHING;

-- Employee user
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000013',
    '00000000-0000-0000-0000-000000000001',
    'employee@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Employee',
    'User',
    'employee'
) ON CONFLICT (id) DO NOTHING;

-- ============================================
-- Summary of seeded data:
-- ============================================
-- Organization: Demo Company (slug: demo)
-- Users:
--   superadmin@demo.com | super_admin | Password123!
--   admin@demo.com      | admin       | Password123!
--   manager@demo.com    | manager     | Password123!
--   employee@demo.com   | employee    | Password123!
-- ============================================

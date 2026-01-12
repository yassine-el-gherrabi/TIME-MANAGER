-- ============================================
-- DEMO SEED DATA FOR PRESENTATION
-- TechCorp France - Time Manager
-- ============================================
--
-- Presentation date: January 12, 2026
-- 6 months of history (July 2025 -> January 2026)
-- Timezone: Europe/Paris
-- Password: "Password123!" for all users
--
-- Mode: IDEMPOTENT (ON CONFLICT DO NOTHING)
-- ============================================

-- ============================================
-- 1. ORGANIZATION
-- ============================================
INSERT INTO organizations (id, name, slug, timezone)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'TechCorp France',
    'techcorp',
    'Europe/Paris'
) ON CONFLICT (id) DO UPDATE SET name = 'TechCorp France', slug = 'techcorp';

-- ============================================
-- 2. WORK SCHEDULES (5 schedules)
-- ============================================

-- Standard 35h (Default)
INSERT INTO work_schedules (id, organization_id, name, description, is_default)
VALUES (
    '00000000-0000-0000-0000-000000000020',
    '00000000-0000-0000-0000-000000000001',
    'Standard 35h',
    'Temps plein standard: Lundi-Vendredi, 09h00-17h00 avec 1h de pause',
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO work_schedule_days (id, work_schedule_id, day_of_week, start_time, end_time, break_minutes) VALUES
    ('00000000-0000-0000-0000-000000000101', '00000000-0000-0000-0000-000000000020', 1, '09:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000102', '00000000-0000-0000-0000-000000000020', 2, '09:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000103', '00000000-0000-0000-0000-000000000020', 3, '09:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000104', '00000000-0000-0000-0000-000000000020', 4, '09:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000105', '00000000-0000-0000-0000-000000000020', 5, '09:00', '17:00', 60)
ON CONFLICT (id) DO NOTHING;

-- Flexible 40h
INSERT INTO work_schedules (id, organization_id, name, description, is_default)
VALUES (
    '00000000-0000-0000-0000-000000000021',
    '00000000-0000-0000-0000-000000000001',
    'Flexible 40h',
    'Horaires flexibles: Lundi-Vendredi, 08h00-17h00 avec 1h de pause',
    false
) ON CONFLICT (id) DO NOTHING;

INSERT INTO work_schedule_days (id, work_schedule_id, day_of_week, start_time, end_time, break_minutes) VALUES
    ('00000000-0000-0000-0000-000000000111', '00000000-0000-0000-0000-000000000021', 1, '08:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000112', '00000000-0000-0000-0000-000000000021', 2, '08:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000113', '00000000-0000-0000-0000-000000000021', 3, '08:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000114', '00000000-0000-0000-0000-000000000021', 4, '08:00', '17:00', 60),
    ('00000000-0000-0000-0000-000000000115', '00000000-0000-0000-0000-000000000021', 5, '08:00', '17:00', 60)
ON CONFLICT (id) DO NOTHING;

-- Mi-temps Matin 20h
INSERT INTO work_schedules (id, organization_id, name, description, is_default)
VALUES (
    '00000000-0000-0000-0000-000000000022',
    '00000000-0000-0000-0000-000000000001',
    'Mi-temps Matin',
    'Mi-temps matinal: Lundi-Vendredi, 09h00-13h00',
    false
) ON CONFLICT (id) DO NOTHING;

INSERT INTO work_schedule_days (id, work_schedule_id, day_of_week, start_time, end_time, break_minutes) VALUES
    ('00000000-0000-0000-0000-000000000121', '00000000-0000-0000-0000-000000000022', 1, '09:00', '13:00', 0),
    ('00000000-0000-0000-0000-000000000122', '00000000-0000-0000-0000-000000000022', 2, '09:00', '13:00', 0),
    ('00000000-0000-0000-0000-000000000123', '00000000-0000-0000-0000-000000000022', 3, '09:00', '13:00', 0),
    ('00000000-0000-0000-0000-000000000124', '00000000-0000-0000-0000-000000000022', 4, '09:00', '13:00', 0),
    ('00000000-0000-0000-0000-000000000125', '00000000-0000-0000-0000-000000000022', 5, '09:00', '13:00', 0)
ON CONFLICT (id) DO NOTHING;

-- Mi-temps 3 jours 24h
INSERT INTO work_schedules (id, organization_id, name, description, is_default)
VALUES (
    '00000000-0000-0000-0000-000000000023',
    '00000000-0000-0000-0000-000000000001',
    'Mi-temps 3 jours',
    'Mi-temps 3 jours: Lundi/Mercredi/Vendredi, 09h00-17h00',
    false
) ON CONFLICT (id) DO NOTHING;

INSERT INTO work_schedule_days (id, work_schedule_id, day_of_week, start_time, end_time, break_minutes) VALUES
    ('00000000-0000-0000-0000-000000000131', '00000000-0000-0000-0000-000000000023', 1, '09:00', '17:00', 30),
    ('00000000-0000-0000-0000-000000000133', '00000000-0000-0000-0000-000000000023', 3, '09:00', '17:00', 30),
    ('00000000-0000-0000-0000-000000000135', '00000000-0000-0000-0000-000000000023', 5, '09:00', '17:00', 30)
ON CONFLICT (id) DO NOTHING;

-- Support Soir 35h
INSERT INTO work_schedules (id, organization_id, name, description, is_default)
VALUES (
    '00000000-0000-0000-0000-000000000024',
    '00000000-0000-0000-0000-000000000001',
    'Support Soir',
    'Equipe support soir: Lundi-Vendredi, 14h00-22h00',
    false
) ON CONFLICT (id) DO NOTHING;

INSERT INTO work_schedule_days (id, work_schedule_id, day_of_week, start_time, end_time, break_minutes) VALUES
    ('00000000-0000-0000-0000-000000000141', '00000000-0000-0000-0000-000000000024', 1, '14:00', '22:00', 60),
    ('00000000-0000-0000-0000-000000000142', '00000000-0000-0000-0000-000000000024', 2, '14:00', '22:00', 60),
    ('00000000-0000-0000-0000-000000000143', '00000000-0000-0000-0000-000000000024', 3, '14:00', '22:00', 60),
    ('00000000-0000-0000-0000-000000000144', '00000000-0000-0000-0000-000000000024', 4, '14:00', '22:00', 60),
    ('00000000-0000-0000-0000-000000000145', '00000000-0000-0000-0000-000000000024', 5, '14:00', '22:00', 60)
ON CONFLICT (id) DO NOTHING;

-- ============================================
-- 3. USERS (45 total)
-- ============================================
-- Password: "Password123!" for all users
-- Argon2id hash generated with default parameters

-- === DEMO ACCOUNTS (4) ===

-- Super Admin - demo@timemanager.com
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000010',
    '00000000-0000-0000-0000-000000000001',
    'demo@timemanager.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Demo',
    'Admin',
    'super_admin',
    '00000000-0000-0000-0000-000000000020'
) ON CONFLICT (id) DO NOTHING;

-- Admin (HR) - sophie.bernard@demo.com
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000011',
    '00000000-0000-0000-0000-000000000001',
    'sophie.bernard@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Sophie',
    'Bernard',
    'admin',
    '00000000-0000-0000-0000-000000000020'
) ON CONFLICT (id) DO NOTHING;

-- Manager (Tech) - jean.dupont@demo.com
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000012',
    '00000000-0000-0000-0000-000000000001',
    'jean.dupont@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Jean',
    'Dupont',
    'manager',
    '00000000-0000-0000-0000-000000000020'
) ON CONFLICT (id) DO NOTHING;

-- Employee (Tech) - marie.martin@demo.com
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000013',
    '00000000-0000-0000-0000-000000000001',
    'marie.martin@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Marie',
    'Martin',
    'employee',
    '00000000-0000-0000-0000-000000000020'
) ON CONFLICT (id) DO NOTHING;

-- === MANAGERS (4 additional) ===

-- Manager Sales - Pierre Leroy
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000014',
    '00000000-0000-0000-0000-000000000001',
    'pierre.leroy@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Pierre',
    'Leroy',
    'manager',
    '00000000-0000-0000-0000-000000000021'
) ON CONFLICT (id) DO NOTHING;

-- Manager Marketing - Claire Dubois
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000015',
    '00000000-0000-0000-0000-000000000001',
    'claire.dubois@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Claire',
    'Dubois',
    'manager',
    '00000000-0000-0000-0000-000000000020'
) ON CONFLICT (id) DO NOTHING;

-- Manager Finance - Marc Mercier
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000016',
    '00000000-0000-0000-0000-000000000001',
    'marc.mercier@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Marc',
    'Mercier',
    'manager',
    '00000000-0000-0000-0000-000000000020'
) ON CONFLICT (id) DO NOTHING;

-- Manager Support - Alice Moreau
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id)
VALUES (
    '00000000-0000-0000-0000-000000000017',
    '00000000-0000-0000-0000-000000000001',
    'alice.moreau@demo.com',
    '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA',
    'Alice',
    'Moreau',
    'manager',
    '00000000-0000-0000-0000-000000000024'
) ON CONFLICT (id) DO NOTHING;

-- === TECH TEAM EMPLOYEES (11) ===

INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id) VALUES
    ('00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000001', 'lucas.moreau@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Lucas', 'Moreau', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000021', '00000000-0000-0000-0000-000000000001', 'emma.petit@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Emma', 'Petit', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000022', '00000000-0000-0000-0000-000000000001', 'thomas.garcia@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Thomas', 'Garcia', 'employee', '00000000-0000-0000-0000-000000000021'),
    ('00000000-0000-0000-0000-000000000023', '00000000-0000-0000-0000-000000000001', 'lea.roux@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Lea', 'Roux', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000024', '00000000-0000-0000-0000-000000000001', 'hugo.david@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Hugo', 'David', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000025', '00000000-0000-0000-0000-000000000001', 'chloe.bertrand@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Chloe', 'Bertrand', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000026', '00000000-0000-0000-0000-000000000001', 'louis.morel@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Louis', 'Morel', 'employee', '00000000-0000-0000-0000-000000000021'),
    ('00000000-0000-0000-0000-000000000027', '00000000-0000-0000-0000-000000000001', 'manon.fournier@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Manon', 'Fournier', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000028', '00000000-0000-0000-0000-000000000001', 'paul.girard@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Paul', 'Girard', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000029', '00000000-0000-0000-0000-000000000001', 'sarah.bonnet@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Sarah', 'Bonnet', 'employee', '00000000-0000-0000-0000-000000000022'),
    ('00000000-0000-0000-0000-000000000030', '00000000-0000-0000-0000-000000000001', 'arthur.lambert@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Arthur', 'Lambert', 'employee', '00000000-0000-0000-0000-000000000020')
ON CONFLICT (id) DO NOTHING;

-- === SALES TEAM EMPLOYEES (7) ===

INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id) VALUES
    ('00000000-0000-0000-0000-000000000031', '00000000-0000-0000-0000-000000000001', 'louise.fontaine@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Louise', 'Fontaine', 'employee', '00000000-0000-0000-0000-000000000021'),
    ('00000000-0000-0000-0000-000000000032', '00000000-0000-0000-0000-000000000001', 'gabriel.rousseau@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Gabriel', 'Rousseau', 'employee', '00000000-0000-0000-0000-000000000021'),
    ('00000000-0000-0000-0000-000000000033', '00000000-0000-0000-0000-000000000001', 'jade.blanc@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Jade', 'Blanc', 'employee', '00000000-0000-0000-0000-000000000021'),
    ('00000000-0000-0000-0000-000000000034', '00000000-0000-0000-0000-000000000001', 'raphael.guerin@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Raphael', 'Guerin', 'employee', '00000000-0000-0000-0000-000000000021'),
    ('00000000-0000-0000-0000-000000000035', '00000000-0000-0000-0000-000000000001', 'alice.muller@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Alice', 'Muller', 'employee', '00000000-0000-0000-0000-000000000021'),
    ('00000000-0000-0000-0000-000000000036', '00000000-0000-0000-0000-000000000001', 'leo.henry@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Leo', 'Henry', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000037', '00000000-0000-0000-0000-000000000001', 'charlotte.masson@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Charlotte', 'Masson', 'employee', '00000000-0000-0000-0000-000000000020')
ON CONFLICT (id) DO NOTHING;

-- === MARKETING TEAM EMPLOYEES (5) ===

INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id) VALUES
    ('00000000-0000-0000-0000-000000000038', '00000000-0000-0000-0000-000000000001', 'adam.chevalier@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Adam', 'Chevalier', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000039', '00000000-0000-0000-0000-000000000001', 'clara.fernandez@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Clara', 'Fernandez', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000040', '00000000-0000-0000-0000-000000000001', 'nathan.lemaire@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Nathan', 'Lemaire', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000041', '00000000-0000-0000-0000-000000000001', 'ines.marchand@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Ines', 'Marchand', 'employee', '00000000-0000-0000-0000-000000000022'),
    ('00000000-0000-0000-0000-000000000042', '00000000-0000-0000-0000-000000000001', 'mathis.duval@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Mathis', 'Duval', 'employee', '00000000-0000-0000-0000-000000000020')
ON CONFLICT (id) DO NOTHING;

-- === HR TEAM EMPLOYEES (3) ===

INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id) VALUES
    ('00000000-0000-0000-0000-000000000043', '00000000-0000-0000-0000-000000000001', 'laura.martinez@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Laura', 'Martinez', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000044', '00000000-0000-0000-0000-000000000001', 'theo.lopez@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Theo', 'Lopez', 'employee', '00000000-0000-0000-0000-000000000023'),
    ('00000000-0000-0000-0000-000000000045', '00000000-0000-0000-0000-000000000001', 'juliette.gonzalez@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Juliette', 'Gonzalez', 'employee', '00000000-0000-0000-0000-000000000020')
ON CONFLICT (id) DO NOTHING;

-- === FINANCE TEAM EMPLOYEES (4) ===

INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id) VALUES
    ('00000000-0000-0000-0000-000000000046', '00000000-0000-0000-0000-000000000001', 'maxime.sanchez@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Maxime', 'Sanchez', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000047', '00000000-0000-0000-0000-000000000001', 'zoe.perez@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Zoe', 'Perez', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000048', '00000000-0000-0000-0000-000000000001', 'ethan.martin@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Ethan', 'Martin', 'employee', '00000000-0000-0000-0000-000000000020'),
    ('00000000-0000-0000-0000-000000000049', '00000000-0000-0000-0000-000000000001', 'pauline.durand@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Pauline', 'Durand', 'employee', '00000000-0000-0000-0000-000000000020')
ON CONFLICT (id) DO NOTHING;

-- === SUPPORT TEAM EMPLOYEES (6) ===

INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role, work_schedule_id) VALUES
    ('00000000-0000-0000-0000-000000000050', '00000000-0000-0000-0000-000000000001', 'nolan.lefebvre@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Nolan', 'Lefebvre', 'employee', '00000000-0000-0000-0000-000000000024'),
    ('00000000-0000-0000-0000-000000000051', '00000000-0000-0000-0000-000000000001', 'anais.legrand@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Anais', 'Legrand', 'employee', '00000000-0000-0000-0000-000000000024'),
    ('00000000-0000-0000-0000-000000000052', '00000000-0000-0000-0000-000000000001', 'jules.garnier@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Jules', 'Garnier', 'employee', '00000000-0000-0000-0000-000000000024'),
    ('00000000-0000-0000-0000-000000000053', '00000000-0000-0000-0000-000000000001', 'lina.faure@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Lina', 'Faure', 'employee', '00000000-0000-0000-0000-000000000024'),
    ('00000000-0000-0000-0000-000000000054', '00000000-0000-0000-0000-000000000001', 'robin.robin@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Robin', 'Robin', 'employee', '00000000-0000-0000-0000-000000000023'),
    ('00000000-0000-0000-0000-000000000055', '00000000-0000-0000-0000-000000000001', 'eva.clement@demo.com', '$argon2id$v=19$m=19456,t=2,p=1$Rt6XiUq41JMErumeJ5qbNw$JZrbCaF+dv38aIYcg/lO4eKcVBvAUk0HQFUjqVm1ufA', 'Eva', 'Clement', 'employee', '00000000-0000-0000-0000-000000000024')
ON CONFLICT (id) DO NOTHING;

-- ============================================
-- 4. TEAMS (6 teams)
-- ============================================

INSERT INTO teams (id, organization_id, name, description, manager_id) VALUES
    ('00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000001', 'Tech', 'Equipe developpement et infrastructure', '00000000-0000-0000-0000-000000000012'),
    ('00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000001', 'Sales', 'Equipe commerciale', '00000000-0000-0000-0000-000000000014'),
    ('00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000001', 'Marketing', 'Equipe marketing et communication', '00000000-0000-0000-0000-000000000015'),
    ('00000000-0000-0000-0000-000000000063', '00000000-0000-0000-0000-000000000001', 'HR', 'Ressources humaines', '00000000-0000-0000-0000-000000000011'),
    ('00000000-0000-0000-0000-000000000064', '00000000-0000-0000-0000-000000000001', 'Finance', 'Comptabilite et finance', '00000000-0000-0000-0000-000000000016'),
    ('00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000001', 'Support', 'Support client', '00000000-0000-0000-0000-000000000017')
ON CONFLICT (id) DO NOTHING;

-- ============================================
-- 5. TEAM MEMBERS
-- ============================================

-- Tech Team (12 members: 1 manager + 11 employees + marie.martin)
INSERT INTO team_members (id, team_id, user_id, joined_at) VALUES
    ('00000000-0000-0000-0000-000000000200', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000012', '2025-01-15'),
    ('00000000-0000-0000-0000-000000000201', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000013', '2025-02-01'),
    ('00000000-0000-0000-0000-000000000202', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000020', '2025-03-01'),
    ('00000000-0000-0000-0000-000000000203', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000021', '2025-03-15'),
    ('00000000-0000-0000-0000-000000000204', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000022', '2025-04-01'),
    ('00000000-0000-0000-0000-000000000205', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000023', '2025-04-15'),
    ('00000000-0000-0000-0000-000000000206', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000024', '2025-05-01'),
    ('00000000-0000-0000-0000-000000000207', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000025', '2025-05-15'),
    ('00000000-0000-0000-0000-000000000208', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000026', '2025-06-01'),
    ('00000000-0000-0000-0000-000000000209', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000027', '2025-06-15'),
    ('00000000-0000-0000-0000-000000000210', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000028', '2025-07-01'),
    ('00000000-0000-0000-0000-000000000211', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000029', '2025-07-15'),
    ('00000000-0000-0000-0000-000000000212', '00000000-0000-0000-0000-000000000060', '00000000-0000-0000-0000-000000000030', '2025-08-01')
ON CONFLICT (id) DO NOTHING;

-- Sales Team (8 members)
INSERT INTO team_members (id, team_id, user_id, joined_at) VALUES
    ('00000000-0000-0000-0000-000000000220', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000014', '2025-01-15'),
    ('00000000-0000-0000-0000-000000000221', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000031', '2025-02-01'),
    ('00000000-0000-0000-0000-000000000222', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000032', '2025-02-15'),
    ('00000000-0000-0000-0000-000000000223', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000033', '2025-03-01'),
    ('00000000-0000-0000-0000-000000000224', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000034', '2025-04-01'),
    ('00000000-0000-0000-0000-000000000225', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000035', '2025-05-01'),
    ('00000000-0000-0000-0000-000000000226', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000036', '2025-06-01'),
    ('00000000-0000-0000-0000-000000000227', '00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000037', '2025-07-01')
ON CONFLICT (id) DO NOTHING;

-- Marketing Team (6 members)
INSERT INTO team_members (id, team_id, user_id, joined_at) VALUES
    ('00000000-0000-0000-0000-000000000230', '00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000015', '2025-01-15'),
    ('00000000-0000-0000-0000-000000000231', '00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000038', '2025-02-01'),
    ('00000000-0000-0000-0000-000000000232', '00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000039', '2025-03-01'),
    ('00000000-0000-0000-0000-000000000233', '00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000040', '2025-04-01'),
    ('00000000-0000-0000-0000-000000000234', '00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000041', '2025-05-01'),
    ('00000000-0000-0000-0000-000000000235', '00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000042', '2025-06-01')
ON CONFLICT (id) DO NOTHING;

-- HR Team (4 members)
INSERT INTO team_members (id, team_id, user_id, joined_at) VALUES
    ('00000000-0000-0000-0000-000000000240', '00000000-0000-0000-0000-000000000063', '00000000-0000-0000-0000-000000000011', '2025-01-15'),
    ('00000000-0000-0000-0000-000000000241', '00000000-0000-0000-0000-000000000063', '00000000-0000-0000-0000-000000000043', '2025-02-01'),
    ('00000000-0000-0000-0000-000000000242', '00000000-0000-0000-0000-000000000063', '00000000-0000-0000-0000-000000000044', '2025-03-01'),
    ('00000000-0000-0000-0000-000000000243', '00000000-0000-0000-0000-000000000063', '00000000-0000-0000-0000-000000000045', '2025-04-01')
ON CONFLICT (id) DO NOTHING;

-- Finance Team (5 members)
INSERT INTO team_members (id, team_id, user_id, joined_at) VALUES
    ('00000000-0000-0000-0000-000000000250', '00000000-0000-0000-0000-000000000064', '00000000-0000-0000-0000-000000000016', '2025-01-15'),
    ('00000000-0000-0000-0000-000000000251', '00000000-0000-0000-0000-000000000064', '00000000-0000-0000-0000-000000000046', '2025-02-01'),
    ('00000000-0000-0000-0000-000000000252', '00000000-0000-0000-0000-000000000064', '00000000-0000-0000-0000-000000000047', '2025-03-01'),
    ('00000000-0000-0000-0000-000000000253', '00000000-0000-0000-0000-000000000064', '00000000-0000-0000-0000-000000000048', '2025-04-01'),
    ('00000000-0000-0000-0000-000000000254', '00000000-0000-0000-0000-000000000064', '00000000-0000-0000-0000-000000000049', '2025-05-01')
ON CONFLICT (id) DO NOTHING;

-- Support Team (7 members)
INSERT INTO team_members (id, team_id, user_id, joined_at) VALUES
    ('00000000-0000-0000-0000-000000000260', '00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000017', '2025-01-15'),
    ('00000000-0000-0000-0000-000000000261', '00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000050', '2025-02-01'),
    ('00000000-0000-0000-0000-000000000262', '00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000051', '2025-03-01'),
    ('00000000-0000-0000-0000-000000000263', '00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000052', '2025-04-01'),
    ('00000000-0000-0000-0000-000000000264', '00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000053', '2025-05-01'),
    ('00000000-0000-0000-0000-000000000265', '00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000054', '2025-06-01'),
    ('00000000-0000-0000-0000-000000000266', '00000000-0000-0000-0000-000000000065', '00000000-0000-0000-0000-000000000055', '2025-07-01')
ON CONFLICT (id) DO NOTHING;

-- ============================================
-- 6. ABSENCE TYPES (6 types)
-- ============================================

INSERT INTO absence_types (id, organization_id, name, code, color, requires_approval, affects_balance, is_paid) VALUES
    ('00000000-0000-0000-0000-000000000501', '00000000-0000-0000-0000-000000000001', 'Conges Payes', 'CP', '#10B981', true, true, true),
    ('00000000-0000-0000-0000-000000000502', '00000000-0000-0000-0000-000000000001', 'Maladie', 'MAL', '#EF4444', true, false, true),
    ('00000000-0000-0000-0000-000000000503', '00000000-0000-0000-0000-000000000001', 'Sans Solde', 'SS', '#F59E0B', true, false, false),
    ('00000000-0000-0000-0000-000000000504', '00000000-0000-0000-0000-000000000001', 'Teletravail', 'TT', '#3B82F6', false, false, true),
    ('00000000-0000-0000-0000-000000000505', '00000000-0000-0000-0000-000000000001', 'Formation', 'FOR', '#8B5CF6', true, false, true),
    ('00000000-0000-0000-0000-000000000506', '00000000-0000-0000-0000-000000000001', 'RTT', 'RTT', '#EC4899', true, true, true)
ON CONFLICT (id) DO NOTHING;

-- ============================================
-- 7. CLOSED DAYS (French holidays 2025-2026)
-- ============================================

INSERT INTO closed_days (id, organization_id, name, date, is_recurring) VALUES
    -- 2025
    ('00000000-0000-0000-0000-000000000601', '00000000-0000-0000-0000-000000000001', 'Jour de l''An 2025', '2025-01-01', false),
    ('00000000-0000-0000-0000-000000000602', '00000000-0000-0000-0000-000000000001', 'Lundi de Paques 2025', '2025-04-21', false),
    ('00000000-0000-0000-0000-000000000603', '00000000-0000-0000-0000-000000000001', 'Fete du Travail 2025', '2025-05-01', false),
    ('00000000-0000-0000-0000-000000000604', '00000000-0000-0000-0000-000000000001', 'Victoire 1945', '2025-05-08', false),
    ('00000000-0000-0000-0000-000000000605', '00000000-0000-0000-0000-000000000001', 'Ascension 2025', '2025-05-29', false),
    ('00000000-0000-0000-0000-000000000606', '00000000-0000-0000-0000-000000000001', 'Lundi Pentecote 2025', '2025-06-09', false),
    ('00000000-0000-0000-0000-000000000607', '00000000-0000-0000-0000-000000000001', 'Fete Nationale 2025', '2025-07-14', false),
    ('00000000-0000-0000-0000-000000000608', '00000000-0000-0000-0000-000000000001', 'Assomption 2025', '2025-08-15', false),
    ('00000000-0000-0000-0000-000000000609', '00000000-0000-0000-0000-000000000001', 'Toussaint 2025', '2025-11-01', false),
    ('00000000-0000-0000-0000-000000000610', '00000000-0000-0000-0000-000000000001', 'Armistice 2025', '2025-11-11', false),
    ('00000000-0000-0000-0000-000000000611', '00000000-0000-0000-0000-000000000001', 'Noel 2025', '2025-12-25', false),
    -- 2026
    ('00000000-0000-0000-0000-000000000612', '00000000-0000-0000-0000-000000000001', 'Jour de l''An 2026', '2026-01-01', false),
    ('00000000-0000-0000-0000-000000000613', '00000000-0000-0000-0000-000000000001', 'Lundi de Paques 2026', '2026-04-06', false),
    ('00000000-0000-0000-0000-000000000614', '00000000-0000-0000-0000-000000000001', 'Fete du Travail 2026', '2026-05-01', false),
    ('00000000-0000-0000-0000-000000000615', '00000000-0000-0000-0000-000000000001', 'Victoire 1945 2026', '2026-05-08', false),
    ('00000000-0000-0000-0000-000000000616', '00000000-0000-0000-0000-000000000001', 'Ascension 2026', '2026-05-14', false),
    ('00000000-0000-0000-0000-000000000617', '00000000-0000-0000-0000-000000000001', 'Lundi Pentecote 2026', '2026-05-25', false),
    ('00000000-0000-0000-0000-000000000618', '00000000-0000-0000-0000-000000000001', 'Fete Nationale 2026', '2026-07-14', false),
    ('00000000-0000-0000-0000-000000000619', '00000000-0000-0000-0000-000000000001', 'Assomption 2026', '2026-08-15', false),
    ('00000000-0000-0000-0000-000000000620', '00000000-0000-0000-0000-000000000001', 'Toussaint 2026', '2026-11-01', false),
    ('00000000-0000-0000-0000-000000000621', '00000000-0000-0000-0000-000000000001', 'Armistice 2026', '2026-11-11', false),
    ('00000000-0000-0000-0000-000000000622', '00000000-0000-0000-0000-000000000001', 'Noel 2026', '2026-12-25', false)
ON CONFLICT (id) DO NOTHING;

-- ============================================
-- 8. LEAVE BALANCES (2025 and 2026)
-- ============================================

-- Generate leave balances for all employees
DO $$
DECLARE
    emp_id UUID;
    emp_ids UUID[] := ARRAY[
        '00000000-0000-0000-0000-000000000013', -- marie.martin
        '00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000021',
        '00000000-0000-0000-0000-000000000022', '00000000-0000-0000-0000-000000000023',
        '00000000-0000-0000-0000-000000000024', '00000000-0000-0000-0000-000000000025',
        '00000000-0000-0000-0000-000000000026', '00000000-0000-0000-0000-000000000027',
        '00000000-0000-0000-0000-000000000028', '00000000-0000-0000-0000-000000000029',
        '00000000-0000-0000-0000-000000000030', '00000000-0000-0000-0000-000000000031',
        '00000000-0000-0000-0000-000000000032', '00000000-0000-0000-0000-000000000033',
        '00000000-0000-0000-0000-000000000034', '00000000-0000-0000-0000-000000000035',
        '00000000-0000-0000-0000-000000000036', '00000000-0000-0000-0000-000000000037',
        '00000000-0000-0000-0000-000000000038', '00000000-0000-0000-0000-000000000039',
        '00000000-0000-0000-0000-000000000040', '00000000-0000-0000-0000-000000000041',
        '00000000-0000-0000-0000-000000000042', '00000000-0000-0000-0000-000000000043',
        '00000000-0000-0000-0000-000000000044', '00000000-0000-0000-0000-000000000045',
        '00000000-0000-0000-0000-000000000046', '00000000-0000-0000-0000-000000000047',
        '00000000-0000-0000-0000-000000000048', '00000000-0000-0000-0000-000000000049',
        '00000000-0000-0000-0000-000000000050', '00000000-0000-0000-0000-000000000051',
        '00000000-0000-0000-0000-000000000052', '00000000-0000-0000-0000-000000000053',
        '00000000-0000-0000-0000-000000000054', '00000000-0000-0000-0000-000000000055'
    ];
    used_days DECIMAL;
BEGIN
    FOREACH emp_id IN ARRAY emp_ids LOOP
        -- Random used days between 0 and 15
        used_days := floor(random() * 15);

        -- Conges Payes 2025
        INSERT INTO leave_balances (id, organization_id, user_id, absence_type_id, year, initial_balance, used, adjustment)
        VALUES (
            gen_random_uuid(),
            '00000000-0000-0000-0000-000000000001',
            emp_id,
            '00000000-0000-0000-0000-000000000501',
            2025,
            25.0,
            used_days,
            0.0
        ) ON CONFLICT DO NOTHING;

        -- RTT 2025
        INSERT INTO leave_balances (id, organization_id, user_id, absence_type_id, year, initial_balance, used, adjustment)
        VALUES (
            gen_random_uuid(),
            '00000000-0000-0000-0000-000000000001',
            emp_id,
            '00000000-0000-0000-0000-000000000506',
            2025,
            12.0,
            floor(random() * 8),
            0.0
        ) ON CONFLICT DO NOTHING;

        -- Conges Payes 2026 (fresh)
        INSERT INTO leave_balances (id, organization_id, user_id, absence_type_id, year, initial_balance, used, adjustment)
        VALUES (
            gen_random_uuid(),
            '00000000-0000-0000-0000-000000000001',
            emp_id,
            '00000000-0000-0000-0000-000000000501',
            2026,
            25.0,
            0.0,
            0.0
        ) ON CONFLICT DO NOTHING;

        -- RTT 2026 (fresh)
        INSERT INTO leave_balances (id, organization_id, user_id, absence_type_id, year, initial_balance, used, adjustment)
        VALUES (
            gen_random_uuid(),
            '00000000-0000-0000-0000-000000000001',
            emp_id,
            '00000000-0000-0000-0000-000000000506',
            2026,
            12.0,
            0.0,
            0.0
        ) ON CONFLICT DO NOTHING;
    END LOOP;
END $$;

-- Special balance for Marie Martin (demo employee) - 8 days used
UPDATE leave_balances
SET used = 8.0
WHERE user_id = '00000000-0000-0000-0000-000000000013'
  AND absence_type_id = '00000000-0000-0000-0000-000000000501'
  AND year = 2025;

-- ============================================
-- 9. GENERATE CLOCK ENTRIES (6 months history)
-- Using generate_series for reliable execution
-- ============================================

-- Generate clock entries for July 2025 to January 2026
-- Uses simpler set-based approach instead of PL/pgSQL loops
INSERT INTO clock_entries (user_id, organization_id, clock_in, clock_out, status)
SELECT
    u.id,
    '00000000-0000-0000-0000-000000000001'::uuid,
    (d.day + ('08:45:00'::time + (floor(random() * 35) || ' minutes')::interval))::timestamptz,
    (d.day + ('17:00:00'::time + (floor(random() * 45) || ' minutes')::interval))::timestamptz,
    CASE
        WHEN d.day >= '2026-01-06' AND random() < 0.6 THEN 'pending'
        WHEN random() < 0.03 THEN 'rejected'
        ELSE 'approved'
    END::clock_entry_status
FROM users u
CROSS JOIN (
    SELECT d::date as day
    FROM generate_series('2025-07-01'::date, '2026-01-10'::date, '1 day') d
    WHERE EXTRACT(DOW FROM d) NOT IN (0, 6)  -- Exclude weekends
    AND d NOT IN (
        '2025-07-14', '2025-08-15', '2025-11-01', '2025-11-11',
        '2025-12-25', '2026-01-01'  -- French holidays
    )
) d
WHERE u.organization_id = '00000000-0000-0000-0000-000000000001'
AND u.role != 'super_admin'
AND random() < 0.95  -- 95% attendance rate
ON CONFLICT DO NOTHING;

-- ============================================
-- 10. ACTIVE CLOCK SESSIONS (5 users currently clocked in)
-- For the Presence Widget demo
-- ============================================

-- Delete any existing active sessions for today and insert fresh ones
DELETE FROM clock_entries
WHERE clock_out IS NULL
  AND DATE(clock_in) = CURRENT_DATE
  AND user_id IN (
    '00000000-0000-0000-0000-000000000013',
    '00000000-0000-0000-0000-000000000020',
    '00000000-0000-0000-0000-000000000021',
    '00000000-0000-0000-0000-000000000024',
    '00000000-0000-0000-0000-000000000025'
  );

-- Marie Martin - clocked in at 9:02 (demo employee)
INSERT INTO clock_entries (id, organization_id, user_id, clock_in, clock_out, status, notes)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000013',
    CURRENT_DATE + TIME '09:02',
    NULL,
    'pending',
    NULL
);

-- Lucas Moreau - clocked in at 8:55
INSERT INTO clock_entries (id, organization_id, user_id, clock_in, clock_out, status, notes)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000020',
    CURRENT_DATE + TIME '08:55',
    NULL,
    'pending',
    NULL
);

-- Emma Petit - clocked in at 9:10
INSERT INTO clock_entries (id, organization_id, user_id, clock_in, clock_out, status, notes)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000021',
    CURRENT_DATE + TIME '09:10',
    NULL,
    'pending',
    NULL
);

-- Hugo David - clocked in at 8:45
INSERT INTO clock_entries (id, organization_id, user_id, clock_in, clock_out, status, notes)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000024',
    CURRENT_DATE + TIME '08:45',
    NULL,
    'pending',
    NULL
);

-- Chloe Bertrand - clocked in at 9:00
INSERT INTO clock_entries (id, organization_id, user_id, clock_in, clock_out, status, notes)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000025',
    CURRENT_DATE + TIME '09:00',
    NULL,
    'pending',
    NULL
);

-- ============================================
-- 11. ABSENCES (mix of statuses for demo)
-- ============================================

-- Marie Martin - pending vacation request (for employee demo)
INSERT INTO absences (id, organization_id, user_id, type_id, start_date, end_date, days_count, status, reason)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000013',
    '00000000-0000-0000-0000-000000000501',
    '2026-01-19',
    '2026-01-23',
    5.0,
    'pending',
    'Vacances familiales'
) ON CONFLICT DO NOTHING;

-- Lucas Moreau - pending absence (for manager approval demo)
INSERT INTO absences (id, organization_id, user_id, type_id, start_date, end_date, days_count, status, reason)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000020',
    '00000000-0000-0000-0000-000000000501',
    '2026-02-02',
    '2026-02-06',
    5.0,
    'pending',
    'Conges hiver'
) ON CONFLICT DO NOTHING;

-- Emma Petit - pending formation
INSERT INTO absences (id, organization_id, user_id, type_id, start_date, end_date, days_count, status, reason)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000021',
    '00000000-0000-0000-0000-000000000505',
    '2026-01-26',
    '2026-01-28',
    3.0,
    'pending',
    'Formation React avancee'
) ON CONFLICT DO NOTHING;

-- Thomas Garcia - pending RTT
INSERT INTO absences (id, organization_id, user_id, type_id, start_date, end_date, days_count, status, reason)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000022',
    '00000000-0000-0000-0000-000000000506',
    '2026-01-16',
    '2026-01-16',
    1.0,
    'pending',
    'RTT'
) ON CONFLICT DO NOTHING;

-- Past approved absences for calendar display
INSERT INTO absences (id, organization_id, user_id, type_id, start_date, end_date, days_count, status, reason, approved_by, approved_at) VALUES
    (gen_random_uuid(), '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000023', '00000000-0000-0000-0000-000000000501', '2025-12-23', '2025-12-31', 7.0, 'approved', 'Vacances Noel', '00000000-0000-0000-0000-000000000012', '2025-12-01'),
    (gen_random_uuid(), '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000024', '00000000-0000-0000-0000-000000000502', '2025-12-15', '2025-12-17', 3.0, 'approved', 'Grippe', '00000000-0000-0000-0000-000000000012', '2025-12-15'),
    (gen_random_uuid(), '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000025', '00000000-0000-0000-0000-000000000504', '2026-01-08', '2026-01-08', 1.0, 'approved', 'Teletravail exceptionnel', NULL, '2026-01-07'),
    (gen_random_uuid(), '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000026', '00000000-0000-0000-0000-000000000501', '2025-11-10', '2025-11-14', 5.0, 'approved', 'Vacances automne', '00000000-0000-0000-0000-000000000012', '2025-10-15'),
    (gen_random_uuid(), '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000027', '00000000-0000-0000-0000-000000000506', '2025-10-20', '2025-10-20', 1.0, 'approved', 'RTT', '00000000-0000-0000-0000-000000000012', '2025-10-10')
ON CONFLICT DO NOTHING;

-- Rejected absence example
INSERT INTO absences (id, organization_id, user_id, type_id, start_date, end_date, days_count, status, reason, rejection_reason, approved_by, approved_at)
VALUES (
    gen_random_uuid(),
    '00000000-0000-0000-0000-000000000001',
    '00000000-0000-0000-0000-000000000028',
    '00000000-0000-0000-0000-000000000503',
    '2025-12-22',
    '2025-12-24',
    3.0,
    'rejected',
    'Conge sans solde',
    'Periode de forte activite, report demande',
    '00000000-0000-0000-0000-000000000012',
    '2025-12-10'
) ON CONFLICT DO NOTHING;

-- ============================================
-- 12. AUDIT LOGS (200+ entries for demo)
-- ============================================

DO $$
DECLARE
    i INT;
    action_type TEXT;
    entity_type TEXT;
    user_id UUID;
    entity_id UUID;
    created_time TIMESTAMPTZ;
    user_ids UUID[] := ARRAY[
        '00000000-0000-0000-0000-000000000010',
        '00000000-0000-0000-0000-000000000011',
        '00000000-0000-0000-0000-000000000012',
        '00000000-0000-0000-0000-000000000013',
        '00000000-0000-0000-0000-000000000014',
        '00000000-0000-0000-0000-000000000015'
    ];
    actions TEXT[] := ARRAY['create', 'update', 'delete'];
    entities TEXT[] := ARRAY['clock_entry', 'clock_entry', 'clock_entry', 'clock_entry', 'user', 'user', 'absence', 'absence', 'team', 'work_schedule'];
BEGIN
    FOR i IN 1..250 LOOP
        action_type := actions[1 + floor(random() * 3)::INT];
        entity_type := entities[1 + floor(random() * 10)::INT];
        user_id := user_ids[1 + floor(random() * 6)::INT];
        entity_id := gen_random_uuid();

        -- Distribute over last 3 months with more recent activity
        IF i <= 30 THEN
            created_time := NOW() - (random() * INTERVAL '24 hours');
        ELSIF i <= 80 THEN
            created_time := NOW() - (random() * INTERVAL '7 days');
        ELSIF i <= 150 THEN
            created_time := NOW() - (random() * INTERVAL '30 days');
        ELSE
            created_time := NOW() - (random() * INTERVAL '90 days');
        END IF;

        INSERT INTO audit_logs (id, organization_id, user_id, action, entity_type, entity_id, old_values, new_values, ip_address, user_agent, created_at)
        VALUES (
            gen_random_uuid(),
            '00000000-0000-0000-0000-000000000001',
            user_id,
            action_type::audit_action,
            entity_type,
            entity_id,
            CASE WHEN action_type IN ('update', 'delete') THEN '{"status": "pending"}'::JSONB ELSE NULL END,
            CASE WHEN action_type IN ('create', 'update') THEN '{"status": "approved"}'::JSONB ELSE NULL END,
            '192.168.1.' || (1 + floor(random() * 254))::TEXT,
            'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
            created_time
        ) ON CONFLICT DO NOTHING;
    END LOOP;
END $$;

-- ============================================
-- SUMMARY
-- ============================================
-- Organization: TechCorp France
--
-- Demo Accounts:
--   demo@timemanager.com      | super_admin | Demo Admin
--   sophie.bernard@demo.com   | admin       | Sophie Bernard (HR)
--   jean.dupont@demo.com      | manager     | Jean Dupont (Tech)
--   marie.martin@demo.com     | employee    | Marie Martin (Tech)
--
-- Teams (6):
--   Tech (12), Sales (8), Marketing (6), HR (4), Finance (5), Support (7)
--
-- Total Users: 45
-- Work Schedules: 5
-- Absence Types: 6
-- Closed Days: 22 (2025-2026)
-- Clock Entries: ~6000 (6 months history)
-- Active Sessions: 5 (for Presence Widget)
-- Pending Approvals: ~8 clock + 4 absences
-- Audit Logs: 250+
-- ============================================

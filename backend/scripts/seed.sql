-- Seed data for development and testing
-- Password for all users: "Password123!"

-- Test organization
INSERT INTO organizations (id, name, slug, timezone)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'ACME Corporation',
    'acme',
    'Europe/Paris'
);

-- Test users
-- Note: Password hashes are for "Password123!" using Argon2id
-- These hashes should be regenerated in production

-- Admin user
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000011',
    '00000000-0000-0000-0000-000000000001',
    'admin@acme.com',
    '$argon2id$v=19$m=19456,t=2,p=1$VGVzdFNhbHQxMjM0NTY3$qLml5UNcXzLWmQYt6z3xNhCGBK8F5r0Y7qxZ3fZ4WvQ',
    'John',
    'Doe',
    'admin'
);

-- Manager user
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000012',
    '00000000-0000-0000-0000-000000000001',
    'manager@acme.com',
    '$argon2id$v=19$m=19456,t=2,p=1$VGVzdFNhbHQxMjM0NTY3$qLml5UNcXzLWmQYt6z3xNhCGBK8F5r0Y7qxZ3fZ4WvQ',
    'Jane',
    'Smith',
    'manager'
);

-- Employee user
INSERT INTO users (id, organization_id, email, password_hash, first_name, last_name, role)
VALUES (
    '00000000-0000-0000-0000-000000000013',
    '00000000-0000-0000-0000-000000000001',
    'employee@acme.com',
    '$argon2id$v=19$m=19456,t=2,p=1$VGVzdFNhbHQxMjM0NTY3$qLml5UNcXzLWmQYt6z3xNhCGBK8F5r0Y7qxZ3fZ4WvQ',
    'Bob',
    'Johnson',
    'employee'
);

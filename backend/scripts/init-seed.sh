#!/bin/bash
# ============================================
# INIT SEED SCRIPT FOR PRODUCTION
# Idempotent database seeding for Time Manager
# ============================================

set -e

echo "=========================================="
echo "Time Manager - Database Initialization"
echo "=========================================="

# Configuration
POSTGRES_HOST=${POSTGRES_HOST:-postgres}
POSTGRES_PORT=${POSTGRES_PORT:-5432}
POSTGRES_USER=${POSTGRES_USER:-timemanager}
POSTGRES_DB=${POSTGRES_DB:-timemanager}
MAX_RETRIES=30
RETRY_INTERVAL=2

# Wait for PostgreSQL to be ready
echo "[1/3] Waiting for PostgreSQL at $POSTGRES_HOST:$POSTGRES_PORT..."
retries=0
until pg_isready -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -q; do
    retries=$((retries + 1))
    if [ $retries -ge $MAX_RETRIES ]; then
        echo "ERROR: PostgreSQL not available after $MAX_RETRIES attempts"
        exit 1
    fi
    echo "  Waiting... ($retries/$MAX_RETRIES)"
    sleep $RETRY_INTERVAL
done
echo "  PostgreSQL is ready!"

# Check if schema exists (migrations must have run)
echo "[2/4] Checking if database schema exists..."
SCHEMA_CHECK=$(PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -d "$POSTGRES_DB" -t -c "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'organizations')" 2>/dev/null || echo "f")
SCHEMA_CHECK=$(echo "$SCHEMA_CHECK" | tr -d '[:space:]')

if [ "$SCHEMA_CHECK" != "t" ]; then
    echo "  Schema not ready (organizations table missing)"
    echo "  Waiting for backend to run migrations..."
    echo "=========================================="
    echo "Initialization skipped (schema not ready)"
    echo "=========================================="
    exit 1
fi
echo "  Schema is ready!"

# Check if already seeded (idempotent check)
echo "[3/4] Checking if database is already seeded..."
SEED_CHECK=$(PGPASSWORD="$POSTGRES_PASSWORD" psql -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -d "$POSTGRES_DB" -t -c "SELECT COUNT(*) FROM users WHERE email = 'demo@timemanager.com'" 2>/dev/null || echo "0")
SEED_CHECK=$(echo "$SEED_CHECK" | tr -d '[:space:]')

if [ "$SEED_CHECK" != "0" ] && [ "$SEED_CHECK" != "" ]; then
    echo "  Database already seeded (found demo@timemanager.com)"
    echo "  Skipping seed to preserve existing data."
    echo "=========================================="
    echo "Initialization complete (no changes made)"
    echo "=========================================="
    exit 0
fi

# Run seed
echo "[4/4] Running demo seed..."
echo "  Loading /app/scripts/demo-seed.sql..."

PGPASSWORD="$POSTGRES_PASSWORD" psql \
    -h "$POSTGRES_HOST" \
    -p "$POSTGRES_PORT" \
    -U "$POSTGRES_USER" \
    -d "$POSTGRES_DB" \
    -f /app/scripts/demo-seed.sql \
    -v ON_ERROR_STOP=1

if [ $? -eq 0 ]; then
    echo "  Seed completed successfully!"
else
    echo "ERROR: Seed failed!"
    exit 1
fi

echo "=========================================="
echo "Database initialization complete!"
echo ""
echo "Demo accounts:"
echo "  demo@timemanager.com      (super_admin)"
echo "  sophie.bernard@demo.com   (admin)"
echo "  jean.dupont@demo.com      (manager)"
echo "  marie.martin@demo.com     (employee)"
echo ""
echo "Password: Password123!"
echo "=========================================="

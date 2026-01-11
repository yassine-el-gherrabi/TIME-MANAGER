#!/bin/bash
# ============================================
# entrypoint.dev.sh - Development entrypoint
# Auto-runs migrations and seed on startup
# ============================================

set -e

echo "🚀 Starting Time Manager Backend (Development Mode)"
echo "=================================================="

# Check DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL environment variable is not set"
    exit 1
fi

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
MAX_RETRIES=30
RETRY_COUNT=0

until psql "$DATABASE_URL" -c '\q' 2>/dev/null; do
    RETRY_COUNT=$((RETRY_COUNT + 1))
    if [ $RETRY_COUNT -ge $MAX_RETRIES ]; then
        echo "❌ Failed to connect to PostgreSQL after $MAX_RETRIES attempts"
        exit 1
    fi
    echo "   Attempt $RETRY_COUNT/$MAX_RETRIES - PostgreSQL not ready, retrying in 2s..."
    sleep 2
done

echo "✅ PostgreSQL is ready!"

# Run Diesel migrations
echo ""
echo "📦 Running database migrations..."
if diesel migration run; then
    echo "✅ Migrations completed successfully!"
else
    echo "⚠️  Migration failed or already up to date"
fi

# Run seed data (skip if NO_SEED is set)
if [ -n "$NO_SEED" ]; then
    echo ""
    echo "⏭️  Skipping seed (NO_SEED is set)"
    echo "   Use POST /api/v1/auth/bootstrap to create the first superadmin"
else
    echo ""
    echo "🌱 Seeding database with demo data..."
    SEED_FILE="/app/scripts/seed.sql"

    if [ -f "$SEED_FILE" ]; then
        if psql "$DATABASE_URL" -f "$SEED_FILE" 2>/dev/null; then
            echo "✅ Seed data loaded successfully!"
        else
            echo "⚠️  Seed data already exists or failed to load (this is normal on restart)"
        fi
    else
        echo "⚠️  Seed file not found at $SEED_FILE"
    fi
fi

# Display summary
echo ""
echo "=================================================="
echo "📋 Development Environment Ready!"
echo "=================================================="
if [ -n "$NO_SEED" ]; then
    echo "🆕 Fresh database - no seed data"
    echo "   Bootstrap: POST /api/v1/auth/bootstrap"
else
    echo "🔐 Demo Users (password: Password123!):"
    echo "   - superadmin@demo.com (Super Admin)"
    echo "   - admin@demo.com      (Admin)"
    echo "   - manager@demo.com    (Manager)"
    echo "   - employee@demo.com   (Employee)"
fi
echo ""
echo "🔗 Services:"
echo "   - API:      http://localhost:8000/api"
echo "   - pgAdmin:  http://localhost:5050"
echo "   - Mailpit:  http://localhost:8025"
echo "=================================================="
echo ""

# Start the application with hot reload
echo "🔥 Starting cargo-watch for hot reload..."
exec cargo watch -x run

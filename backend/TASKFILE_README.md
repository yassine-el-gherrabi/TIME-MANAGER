# Taskfile CI Guide

## Installation
```bash
# macOS
brew install go-task

# Linux
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin
```

## Quick Commands

### Development
```bash
task dev:setup          # Complete dev environment setup
task run:dev            # Run with auto-reload
task watch:test         # Watch tests
```

### Testing
```bash
task test               # Run all tests
task test:unit          # Unit tests only
task ci:local           # Full CI pipeline
task ci:quick           # Quick check (no integration tests)
```

### Database
```bash
task db:up              # Start PostgreSQL
task db:migrate         # Run migrations
task db:reset           # Reset database
```

### Code Quality
```bash
task pre-commit         # Run before committing
task fmt                # Format code
task clippy             # Lint code
```

### Phase Validation
```bash
task validate:phase3    # Validate Phase 3
```

## Workflow
```bash
# 1. Setup
task dev:setup

# 2. Development
task watch:test

# 3. Before commit
task pre-commit

# 4. CI validation
task ci:local
```

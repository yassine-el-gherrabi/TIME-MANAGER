# Development Guide

Development workflows, patterns, and best practices for Time Manager.

## Development Workflow

### 1. Feature Development

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes
# ...

# Run tests and linting
cd backend && cargo fmt && cargo clippy
cd frontend && npm run lint && npm run type-check

# Commit changes
git add .
git commit -m "feat: your feature description"

# Push and create PR
git push origin feature/your-feature-name
```

### 2. Code Quality Standards

**Backend (Rust)**:
```bash
# Format code
cargo fmt

# Lint with clippy (no warnings allowed)
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Check for security vulnerabilities
cargo audit
```

**Frontend (TypeScript/React)**:
```bash
# Lint
npm run lint

# Type check
npm run type-check

# Format
npm run format

# Security audit
npm audit
```

## Architecture Patterns

### Backend Structure

```
backend/src/
├── main.rs              # Entry point, server setup
├── lib.rs               # Library root
├── api/                 # HTTP layer
│   ├── router.rs        # Route definitions
│   └── handlers/        # Request handlers
├── config/              # Configuration
│   ├── app.rs          # App config
│   └── database.rs     # DB config
├── domain/              # Business logic
│   └── enums.rs        # Domain enums
├── models/              # Database models
├── services/            # Business services
├── repositories/        # Data access
├── middleware/          # HTTP middleware
├── extractors/          # Request extractors
├── error/               # Error handling
│   └── app_error.rs    # Custom errors
└── utils/               # Utilities
```

### Clean Architecture Principles

1. **Separation of Concerns**
   - API layer handles HTTP
   - Services contain business logic
   - Repositories manage data access

2. **Dependency Injection**
   - Pass dependencies explicitly
   - Use trait objects for flexibility

3. **Error Handling**
   ```rust
   // Use custom error types
   pub enum AppError {
       DatabaseError(diesel::result::Error),
       NotFound(String),
       Unauthorized,
   }
   ```

### Frontend Structure

```
frontend/src/
├── main.tsx             # Entry point
├── App.tsx              # Root component
├── components/          # React components
│   ├── ui/             # Shadcn/UI components
│   ├── layout/         # Layout components
│   └── shared/         # Shared components
├── pages/               # Page components
├── hooks/               # Custom hooks
├── api/                 # API client
├── lib/                 # Utilities
├── types/               # TypeScript types
└── routes/              # Routing configuration
```

### Frontend Patterns

1. **Component Organization**
   ```typescript
   // Feature-based structure
   components/
   ├── auth/
   │   ├── LoginForm.tsx
   │   ├── RegisterForm.tsx
   │   └── index.ts
   ```

2. **Custom Hooks**
   ```typescript
   // hooks/useAuth.ts
   export function useAuth() {
     const [user, setUser] = useState<User | null>(null);
     // ... auth logic
     return { user, login, logout };
   }
   ```

3. **API Client**
   ```typescript
   // api/client.ts
   const client = axios.create({
     baseURL: import.meta.env.VITE_API_URL,
   });
   ```

## Database Migrations

### Creating Migrations

```bash
cd backend

# Create new migration
diesel migration generate create_table_name

# Edit up.sql and down.sql files
# migrations/[timestamp]_create_table_name/up.sql
# migrations/[timestamp]_create_table_name/down.sql

# Run migration
diesel migration run

# Test rollback
diesel migration redo
```

### Migration Best Practices

1. **Always provide rollback** (down.sql)
2. **One logical change per migration**
3. **Test migrations in both directions**
4. **Add indexes for foreign keys**
5. **Use meaningful timestamps and names**

Example migration:
```sql
-- up.sql
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_teams_organization_id ON teams(organization_id);

-- down.sql
DROP TABLE teams;
```

## Testing

### Backend Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        // Arrange
        let user = User::new("test@example.com");

        // Act
        let result = user.validate();

        // Assert
        assert!(result.is_ok());
    }
}
```

### Frontend Tests

```typescript
import { render, screen } from '@testing-library/react';
import { LoginForm } from './LoginForm';

describe('LoginForm', () => {
  it('renders login form', () => {
    render(<LoginForm />);
    expect(screen.getByLabelText('Email')).toBeInTheDocument();
  });
});
```

## Docker Development

### Rebuilding Services

```bash
# Rebuild specific service
docker compose build backend

# Rebuild and restart
docker compose up -d --build backend

# Force rebuild (no cache)
docker compose build --no-cache backend
```

### Viewing Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f backend

# Last 100 lines
docker compose logs --tail=100 backend
```

### Accessing Containers

```bash
# Shell access
docker compose exec backend sh
docker compose exec postgres psql -U timemanager

# Run commands
docker compose exec backend cargo --version
```

## API Development

### Adding New Endpoints

1. **Create Handler**
   ```rust
   // backend/src/api/handlers/users.rs
   pub async fn get_user(
       Path(id): Path<Uuid>,
   ) -> Result<Json<User>, AppError> {
       // Implementation
   }
   ```

2. **Add Route**
   ```rust
   // backend/src/api/router.rs
   pub fn create_router() -> Router {
       Router::new()
           .route("/users/:id", get(get_user))
   }
   ```

3. **Document in README**
   - Method and path
   - Request/response examples
   - Authentication requirements

### API Response Format

```json
{
  "data": { /* response data */ },
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 100
  }
}

// Error response
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Resource not found"
  }
}
```

## Git Workflow

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body (optional)

footer (optional)
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Tests
- `chore`: Build/tooling

**Examples**:
```
feat(auth): add JWT refresh token support

- Add refresh token endpoint
- Update token middleware
- Add refresh token tests

fix(api): correct user validation error messages

docs(readme): update installation instructions

chore(deps): update axios to v1.6.0
```

### Pull Request Process

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/description
   ```

2. **Make Changes** with clear commits

3. **Update Documentation** if needed

4. **Run Quality Checks**
   ```bash
   # Backend
   cargo fmt && cargo clippy && cargo test

   # Frontend
   npm run lint && npm run type-check && npm run build
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/description
   ```

6. **Wait for CI** - All checks must pass

7. **Code Review** - Address feedback

8. **Merge** - Squash or merge as appropriate

## Performance Optimization

### Backend Optimization

1. **Database Queries**
   ```rust
   // Use joins instead of N+1 queries
   users::table
       .inner_join(organizations::table)
       .select((users::all_columns, organizations::name))
       .load(&mut conn)
   ```

2. **Caching**
   - Use Redis for session data
   - Cache frequently accessed data

3. **Async Operations**
   ```rust
   // Use tokio::spawn for independent tasks
   tokio::spawn(async move {
       send_email(user.email).await;
   });
   ```

### Frontend Optimization

1. **Code Splitting**
   ```typescript
   const Dashboard = lazy(() => import('./pages/Dashboard'));
   ```

2. **Memoization**
   ```typescript
   const memoizedValue = useMemo(() =>
       expensiveComputation(data),
       [data]
   );
   ```

3. **Bundle Analysis**
   ```bash
   npm run build -- --analyze
   ```

## Debugging

### Backend Debugging

```bash
# Enable debug logs
RUST_LOG=debug cargo run

# Use rust-lldb (macOS) or rust-gdb (Linux)
rust-lldb target/debug/timemanager-backend
```

### Frontend Debugging

```typescript
// React DevTools
// Install browser extension

// Console debugging
console.log('Debug:', data);

// Breakpoints in browser DevTools
debugger;
```

### Database Debugging

```sql
-- Enable query logging
ALTER DATABASE timemanager SET log_statement = 'all';

-- Check slow queries
SELECT * FROM pg_stat_statements
ORDER BY total_time DESC
LIMIT 10;
```

## Monitoring in Development

```bash
# Watch logs
docker compose logs -f backend

# Monitor resource usage
docker stats

# Check metrics
curl http://localhost:9090/metrics
```

## Security Checklist

- [ ] No secrets in code
- [ ] Environment variables for config
- [ ] Input validation
- [ ] SQL injection protection (use ORM)
- [ ] XSS protection
- [ ] CSRF tokens (for state-changing operations)
- [ ] Rate limiting
- [ ] Security headers
- [ ] HTTPS in production

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [Diesel Guide](https://diesel.rs/guides/)
- [React Docs](https://react.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)

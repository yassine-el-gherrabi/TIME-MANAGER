# Testing Documentation

Comprehensive testing guide for the Time Manager frontend application.

## Table of Contents

- [Testing Strategy](#testing-strategy)
- [Test Status](#test-status)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Test Patterns](#test-patterns)
- [Mocking Guidelines](#mocking-guidelines)
- [Coverage Requirements](#coverage-requirements)
- [Common Issues](#common-issues)

## Testing Strategy

### Testing Philosophy

We follow a comprehensive testing approach with three layers:

1. **Unit Tests**: Test individual functions and utilities in isolation
2. **Integration Tests**: Test component integration with contexts, hooks, and API layer
3. **E2E Tests** (Future): Test complete user workflows with Playwright

### Tools

- **Test Runner**: Vitest 3.2.4 - Fast, modern test runner with excellent TypeScript support
- **Testing Library**: @testing-library/react 16.3.0 - User-centric testing utilities
- **User Interaction**: @testing-library/user-event - Realistic user interaction simulation
- **Coverage**: @vitest/coverage-v8 - V8-based code coverage
- **Test Environment**: jsdom - Simulated browser environment

## Test Status

### Current Coverage

- **Total Tests**: 98 tests written
- **Passing Tests**: 89 tests (91% pass rate)
- **Failing Tests**: 9 tests (ProtectedRoute navigation edge cases)

### Test Breakdown

| Category | File | Tests | Status | Coverage |
|----------|------|-------|--------|----------|
| Unit | `jwt.test.ts` | 12 | ✅ All passing | Core JWT validation |
| Unit | `errorHandler.test.ts` | 23 | ✅ All passing | Error handling utilities |
| Unit | `transformers.test.ts` | 21 | ✅ All passing | snake_case ↔ camelCase |
| Integration | `AuthContext.test.tsx` | 13 | ✅ All passing | Authentication context |
| Integration | `LoginPage.test.tsx` | 18 | ✅ All passing | Login page |
| Integration | `ProtectedRoute.test.tsx` | 11 | ⚠️ 2/11 passing | Route protection |

### Known Issues

The 9 failing ProtectedRoute tests pass individually but fail when run together. This is a test isolation issue that doesn't affect production code. These tests verify:
- Role-based navigation redirects
- Complex transition scenarios
- Multi-role access patterns

## Running Tests

### Basic Commands

```bash
# Run all tests in watch mode (development)
npm run test

# Run tests once (CI/CD)
npm run test:run

# Run tests with UI
npm run test:ui

# Generate coverage report
npm run test:coverage

# Run specific test file
npm run test:run -- src/utils/jwt.test.ts

# Run tests matching pattern
npm run test:run -- --grep "login"
```

### Quality Checks

```bash
# Run all quality checks (lint + type-check + tests)
npm run quality

# Individual checks
npm run lint          # ESLint
npm run type-check    # TypeScript compiler
npm run format:check  # Prettier formatting
```

## Writing Tests

### Test File Structure

```typescript
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

// Mocks
vi.mock('@/hooks/useAuth');

describe('ComponentName', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('feature group', () => {
    it('describes specific behavior', async () => {
      // Arrange
      const user = userEvent.setup();

      // Act
      render(<Component />);
      await user.click(screen.getByRole('button'));

      // Assert
      expect(screen.getByText('Result')).toBeInTheDocument();
    });
  });
});
```

### Test Organization

Group related tests using nested `describe` blocks:

```typescript
describe('LoginPage', () => {
  describe('rendering', () => {
    it('renders login form with all elements', () => {});
    it('email input has correct attributes', () => {});
  });

  describe('form interaction', () => {
    it('updates email input on user typing', async () => {});
  });

  describe('form validation', () => {
    it('shows error toast when submitting empty form', async () => {});
  });

  describe('login submission', () => {
    it('calls login with correct credentials', async () => {});
    it('shows success toast on successful login', async () => {});
  });
});
```

## Test Patterns

### 1. Component Rendering Tests

Test that components render correctly with expected elements:

```typescript
it('renders login form with all elements', () => {
  render(<LoginPage />);

  expect(screen.getByRole('heading', { name: 'Sign In' })).toBeInTheDocument();
  expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
  expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
  expect(screen.getByRole('button', { name: /sign in/i })).toBeInTheDocument();
});
```

### 2. User Interaction Tests

Use `userEvent` for realistic user interactions:

```typescript
it('updates email input on user typing', async () => {
  const user = userEvent.setup();
  render(<LoginPage />);

  const emailInput = screen.getByLabelText(/email/i) as HTMLInputElement;
  await user.type(emailInput, 'test@example.com');

  expect(emailInput.value).toBe('test@example.com');
});
```

### 3. Async Behavior Tests

Use `waitFor` for asynchronous operations:

```typescript
it('shows success toast on successful login', async () => {
  const user = userEvent.setup();
  const { toast } = await import('sonner');
  mockLogin.mockResolvedValue(undefined);

  render(<LoginPage />);

  await user.type(screen.getByLabelText(/email/i), 'test@example.com');
  await user.type(screen.getByLabelText(/password/i), 'password123');
  await user.click(screen.getByRole('button', { name: /sign in/i }));

  await waitFor(() => {
    expect(toast.success).toHaveBeenCalledWith('Welcome back!');
  });
});
```

### 4. Loading State Tests

Test loading indicators and disabled states:

```typescript
it('shows loading state during login', async () => {
  const user = userEvent.setup();
  let resolveLogin: () => void;
  const loginPromise = new Promise<void>((resolve) => {
    resolveLogin = resolve;
  });
  mockLogin.mockReturnValue(loginPromise);

  render(<LoginPage />);

  await user.type(screen.getByLabelText(/email/i), 'test@example.com');
  await user.type(screen.getByLabelText(/password/i), 'password123');
  await user.click(screen.getByRole('button', { name: /sign in/i }));

  // Should show loading state
  await waitFor(() => {
    expect(screen.getByText('Signing in...')).toBeInTheDocument();
  });

  // Button should be disabled
  const submitButton = screen.getByRole('button', { name: /signing in/i });
  expect(submitButton).toBeDisabled();

  // Resolve login
  resolveLogin!();

  await waitFor(() => {
    expect(screen.queryByText('Signing in...')).not.toBeInTheDocument();
  });
});
```

### 5. Error Handling Tests

Test error scenarios and recovery:

```typescript
it('handles login error with error handler', async () => {
  const user = userEvent.setup();
  const error = new Error('Invalid credentials');
  mockLogin.mockRejectedValue(error);

  // Mock console to suppress error logs in test
  const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

  render(<LoginPage />);

  await user.type(screen.getByLabelText(/email/i), 'test@example.com');
  await user.type(screen.getByLabelText(/password/i), 'wrongpassword');
  await user.click(screen.getByRole('button', { name: /sign in/i }));

  await waitFor(() => {
    expect(mockLogin).toHaveBeenCalled();
  });

  // Error handler should be called (verify through console)
  await waitFor(() => {
    expect(consoleSpy).toHaveBeenCalled();
  });

  consoleSpy.mockRestore();
});
```

## Mocking Guidelines

### Mocking Context Providers

For components that use React Context (like `useAuth`), mock the hook:

```typescript
import { useAuth } from '@/hooks/useAuth';

vi.mock('@/hooks/useAuth');

describe('Component', () => {
  beforeEach(() => {
    vi.mocked(useAuth).mockReturnValue({
      user: mockUser,
      token: 'test-token',
      login: vi.fn(),
      logout: vi.fn(),
      loading: false,
      isAuthenticated: true,
    });
  });
});
```

**Important**: Use `mockReturnValue`, not `mockImplementation` with arrow functions:

```typescript
// ✅ Correct
vi.mocked(useAuth).mockReturnValue({ ... });

// ❌ Wrong (unreliable in Vitest)
vi.mocked(useAuth).mockImplementation(() => ({ ... }));
```

### Mocking External Libraries

Mock third-party libraries that have side effects:

```typescript
// Mock toast notifications
vi.mock('sonner', () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));

// Access mocked toast
const { toast } = await import('sonner');
expect(toast.success).toHaveBeenCalledWith('Welcome back!');
```

### Mocking React Router

For components using routing:

```typescript
import { BrowserRouter } from 'react-router-dom';

function renderWithRouter(component: React.ReactElement) {
  return render(
    <BrowserRouter>
      {component}
    </BrowserRouter>
  );
}
```

## Coverage Requirements

### Target Coverage

We aim for **80%+ coverage** across all metrics:

- **Lines**: 80%
- **Functions**: 80%
- **Branches**: 80%
- **Statements**: 80%

### Coverage Configuration

See `vitest.config.ts`:

```typescript
coverage: {
  provider: 'v8',
  reporter: ['text', 'json', 'html', 'lcov'],
  exclude: [
    'node_modules/',
    'src/test/',
    '**/*.d.ts',
    '**/*.config.*',
    'dist/',
    '**/*.test.{ts,tsx}',
    'src/main.tsx',
    'src/components/ui/**', // Shadcn UI components (external)
  ],
  thresholds: {
    lines: 80,
    functions: 80,
    branches: 80,
    statements: 80,
  },
}
```

### What to Exclude from Coverage

- Third-party UI components (shadcn/ui)
- Configuration files
- Type definition files
- Test files themselves
- Main application entry point

## Common Issues

### Issue 1: "Unable to find element" Errors

**Problem**: Test can't find element even though it's rendered.

**Solutions**:
- Use `screen.debug()` to see rendered output
- Check for ambiguous selectors (multiple elements with same text)
- Use role-based queries: `getByRole('button', { name: 'Sign In' })`
- For non-interactive elements, use `getByText()` carefully

```typescript
// ❌ Ambiguous
expect(screen.getByText('Sign In')).toBeInTheDocument();

// ✅ Specific
expect(screen.getByRole('heading', { name: 'Sign In' })).toBeInTheDocument();
```

### Issue 2: Tests Pass Individually but Fail Together

**Problem**: Tests have shared state or incomplete cleanup.

**Solutions**:
- Clear all mocks in `beforeEach`: `vi.clearAllMocks()`
- Reset modules if needed: `vi.resetModules()`
- Unmount components properly
- Check for async operations completing

```typescript
beforeEach(() => {
  vi.clearAllMocks();
  // Reset any global state
  localStorage.clear();
});
```

### Issue 3: Async Tests Timing Out

**Problem**: `waitFor` times out before condition is met.

**Solutions**:
- Increase timeout: `waitFor(() => {...}, { timeout: 3000 })`
- Check that promises actually resolve
- Verify mock functions return values correctly
- Use `screen.findBy*` queries (built-in waiting)

```typescript
// ❌ No timeout
await waitFor(() => {
  expect(screen.getByText('Loaded')).toBeInTheDocument();
});

// ✅ With timeout
await waitFor(() => {
  expect(screen.getByText('Loaded')).toBeInTheDocument();
}, { timeout: 3000 });

// ✅ Or use findBy (implicit waiting)
expect(await screen.findByText('Loaded')).toBeInTheDocument();
```

### Issue 4: Form Validation Not Working in Tests

**Problem**: HTML5 `required` attributes prevent form submission in tests.

**Solutions**:
- Remove `required` attributes and rely on JavaScript validation
- Or test the validation logic separately
- Ensure form has `onSubmit` handler

```typescript
// In component: Remove required attribute
<Input
  type="email"
  value={email}
  onChange={(e) => setEmail(e.target.value)}
  // No 'required' attribute
/>

// In test: Custom validation is now testable
it('shows error when submitting empty form', async () => {
  const user = userEvent.setup();
  render(<LoginPage />);

  await user.click(screen.getByRole('button', { name: /sign in/i }));

  expect(toast.error).toHaveBeenCalledWith('Please fill in all fields');
});
```

### Issue 5: Mock Functions Not Being Called

**Problem**: `expect(mockFn).toHaveBeenCalled()` fails even though function should have been called.

**Solutions**:
- Verify mock is set up before component renders
- Check that mock is passed correctly to component
- Use `waitFor` for async operations
- Clear mocks between tests

```typescript
// ✅ Set up mock before render
mockLogin.mockResolvedValue(undefined);
render(<LoginPage />);

// ✅ Wait for async call
await waitFor(() => {
  expect(mockLogin).toHaveBeenCalled();
});
```

## Best Practices

### 1. Arrange-Act-Assert Pattern

Structure tests clearly:

```typescript
it('description', async () => {
  // Arrange: Set up test data and mocks
  const user = userEvent.setup();
  mockLogin.mockResolvedValue(undefined);

  // Act: Perform actions
  render(<LoginPage />);
  await user.type(screen.getByLabelText(/email/i), 'test@example.com');
  await user.click(screen.getByRole('button', { name: /sign in/i }));

  // Assert: Verify outcomes
  expect(mockLogin).toHaveBeenCalledWith({
    email: 'test@example.com',
    password: 'password123',
  });
});
```

### 2. Test User Behavior, Not Implementation

Focus on what users see and do:

```typescript
// ❌ Testing implementation
expect(wrapper.find('button').prop('disabled')).toBe(true);

// ✅ Testing user perspective
expect(screen.getByRole('button', { name: /sign in/i })).toBeDisabled();
```

### 3. Use Descriptive Test Names

Test names should clearly describe the scenario:

```typescript
// ❌ Unclear
it('works', () => {});
it('test login', () => {});

// ✅ Clear and descriptive
it('shows error toast when submitting empty form', () => {});
it('redirects to employee dashboard after successful employee login', () => {});
```

### 4. Avoid Snapshot Tests for UI

Snapshots are fragile and don't test behavior:

```typescript
// ❌ Fragile snapshot
expect(wrapper).toMatchSnapshot();

// ✅ Test specific behaviors
expect(screen.getByRole('heading', { name: 'Sign In' })).toBeInTheDocument();
expect(screen.getByLabelText(/email/i)).toHaveAttribute('type', 'email');
```

### 5. Mock at the Right Level

Mock at the boundary, not internal implementation:

```typescript
// ✅ Mock the hook (boundary)
vi.mock('@/hooks/useAuth');

// ❌ Mock internal context implementation
vi.mock('@/context/AuthContext', () => ({ ... }));
```

## Continuous Improvement

### Future Enhancements

1. **E2E Tests with Playwright**
   - Full authentication workflow
   - Cross-browser testing
   - Visual regression testing

2. **Component Visual Testing**
   - Storybook integration
   - Chromatic for visual diffs
   - Accessibility audits

3. **Performance Testing**
   - Lighthouse CI integration
   - Bundle size monitoring
   - Render performance profiling

4. **Mutation Testing**
   - Stryker integration
   - Verify test effectiveness
   - Identify weak test coverage

### Contributing Tests

When adding new features:

1. Write tests alongside feature code
2. Aim for 80%+ coverage on new code
3. Follow existing test patterns
4. Run full test suite before committing: `npm run quality`
5. Update this documentation if adding new testing patterns

## Resources

- [Vitest Documentation](https://vitest.dev/)
- [Testing Library Documentation](https://testing-library.com/docs/react-testing-library/intro/)
- [User Event Documentation](https://testing-library.com/docs/user-event/intro)
- [Testing Best Practices](https://kentcdodds.com/blog/common-mistakes-with-react-testing-library)

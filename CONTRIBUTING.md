# Contributing to Time Manager

First off, thank you for considering contributing to Time Manager! It's people like you that make this project a great tool.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Git Workflow](#git-workflow)
- [Commit Message Convention](#commit-message-convention)
- [Pull Request Process](#pull-request-process)
- [Code Standards](#code-standards)
- [Testing Guidelines](#testing-guidelines)
- [Code Review](#code-review)

## 📜 Code of Conduct

This project and everyone participating in it is governed by respect, professionalism, and collaboration. Please be kind and courteous to other contributors.

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/time-manager.git
   cd time-manager
   ```
3. **Add the upstream repository**:
   ```bash
   git remote add upstream https://github.com/EPITECHMSC/time-manager.git
   ```
4. **Install dependencies**:
   ```bash
   # Backend
   cd back && go mod download

   # Frontend
   cd front && npm install
   ```

## 🔄 Development Workflow

### 1. Keep your fork synchronized

```bash
git fetch upstream
git checkout master
git merge upstream/master
```

### 2. Create a feature branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Make your changes

- Write clear, readable code
- Follow the project's coding standards
- Add tests for new functionality
- Update documentation as needed

### 4. Test your changes

```bash
# Backend tests
cd back && go test ./... -v

# Frontend tests
cd front && npm test

# Linting
cd back && golangci-lint run
cd front && npm run lint
```

### 5. Commit your changes

Follow our [commit message convention](#commit-message-convention).

### 6. Push to your fork

```bash
git push origin feature/your-feature-name
```

### 7. Open a Pull Request

Go to the original repository and click "New Pull Request".

## 🌿 Git Workflow

We use a **feature branch workflow** with the following conventions:

### Branch Naming

- `feature/description` - For new features
- `fix/description` - For bug fixes
- `docs/description` - For documentation updates
- `refactor/description` - For code refactoring
- `test/description` - For adding or updating tests
- `chore/description` - For maintenance tasks

**Examples**:
- `feature/add-user-authentication`
- `fix/resolve-clock-in-bug`
- `docs/update-api-documentation`

### Branch Protection

The `master` branch is protected and requires:
- ✅ All status checks passing (CI/CD)
- ✅ At least one approval from code reviewer
- ✅ All commits following conventional commit format
- ✅ No merge conflicts

## 💬 Commit Message Convention

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature for the user
- `fix`: Bug fix for the user
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring (no functional changes)
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `build`: Changes to build system or dependencies
- `ci`: Changes to CI/CD configuration
- `chore`: Other changes that don't modify src or test files
- `revert`: Revert a previous commit

### Scopes (optional but recommended)

- `auth`: Authentication related
- `api`: API endpoints
- `ui`: User interface
- `database`: Database operations
- `config`: Configuration files
- `clock`: Clock in/out functionality
- `team`: Team management
- `user`: User management
- `report`: Reporting and KPIs

### Rules

- Use imperative, present tense: "add" not "added" nor "adds"
- Don't capitalize first letter of subject
- No period (.) at the end of subject
- Subject line maximum 72 characters
- Separate subject from body with blank line
- Wrap body at 72 characters
- Explain **what** and **why** vs. **how** in the body

### Examples

#### Good commits

```bash
feat(auth): add JWT authentication system

Implement JWT-based authentication with access and refresh tokens.
Includes middleware for route protection and role-based access control.

Closes #42

fix(clock): resolve timezone issue in clock-out

The clock-out timestamp was not correctly accounting for user timezone.
Fixed by converting to UTC before storing in database.

docs: update installation instructions in README

test(api): add unit tests for user CRUD operations

refactor(database): optimize query performance for reports
```

#### Bad commits

```bash
Update files        # Too vague
Fixed stuff         # Not descriptive
WIP                # Work in progress is not acceptable
Added new feature   # Past tense instead of imperative
```

## 🔀 Pull Request Process

### Before Submitting a PR

- [ ] Code follows the project's style guidelines
- [ ] Self-review of your code completed
- [ ] Comments added to hard-to-understand areas
- [ ] Documentation updated (if applicable)
- [ ] No new warnings generated
- [ ] Tests added/updated and all tests pass
- [ ] Commits follow conventional commit format
- [ ] Branch is up to date with `master`

### PR Title

Use the same format as commit messages:
```
feat(auth): add JWT authentication system
```

### PR Description

Use the PR template provided. Include:

1. **Description**: What does this PR do?
2. **Type of Change**: Feature, bug fix, docs, etc.
3. **Testing**: How was this tested?
4. **Screenshots**: If applicable (UI changes)
5. **Related Issues**: Link to issues this PR addresses
6. **Checklist**: Complete all items

### PR Review Process

1. **Automated Checks**: All CI/CD checks must pass
2. **Code Review**: At least one approval required
3. **Testing**: Reviewers may test the changes locally
4. **Feedback**: Address all comments and questions
5. **Approval**: PR can be merged once approved

### After PR is Merged

1. Delete your feature branch:
   ```bash
   git branch -d feature/your-feature-name
   git push origin --delete feature/your-feature-name
   ```
2. Update your local master:
   ```bash
   git checkout master
   git pull upstream master
   ```

## 🎨 Code Standards

### Backend (Go)

- Follow [Effective Go](https://golang.org/doc/effective_go) guidelines
- Use `gofmt` for formatting (automatically enforced)
- Use `golangci-lint` for linting
- Keep functions small and focused (< 50 lines ideally)
- Write meaningful variable and function names
- Add comments for exported functions and complex logic
- Handle errors explicitly, never ignore them

**Example**:
```go
// GetUserByID retrieves a user from the database by ID.
// Returns an error if the user is not found or if a database error occurs.
func GetUserByID(id int) (*User, error) {
    var user User
    err := db.QueryRow("SELECT * FROM users WHERE id = $1", id).Scan(&user)
    if err != nil {
        if err == sql.ErrNoRows {
            return nil, ErrUserNotFound
        }
        return nil, fmt.Errorf("failed to get user: %w", err)
    }
    return &user, nil
}
```

### Frontend (React)

- Follow [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)
- Use ESLint and Prettier (automatically enforced)
- Prefer functional components with hooks over class components
- Keep components small and reusable (< 300 lines)
- Use meaningful prop names and PropTypes/TypeScript
- Extract complex logic into custom hooks
- Follow the project folder structure

**Example**:
```jsx
// components/common/Button.jsx
import React from 'react';
import PropTypes from 'prop-types';

/**
 * Reusable Button component with multiple variants
 */
const Button = ({ children, variant = 'primary', onClick, disabled }) => {
  return (
    <button
      className={`btn btn-${variant}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
};

Button.propTypes = {
  children: PropTypes.node.isRequired,
  variant: PropTypes.oneOf(['primary', 'secondary', 'danger']),
  onClick: PropTypes.func,
  disabled: PropTypes.bool
};

export default Button;
```

### General Principles

- **DRY**: Don't Repeat Yourself - Extract common functionality
- **KISS**: Keep It Simple, Stupid - Prefer simplicity over cleverness
- **YAGNI**: You Aren't Gonna Need It - Don't add features speculatively
- **Single Responsibility**: Each function/component should do one thing well
- **Meaningful Names**: Use descriptive names, avoid abbreviations

## 🧪 Testing Guidelines

### Backend Testing

- Write unit tests for all business logic
- Test both success and error cases
- Use table-driven tests when appropriate
- Mock external dependencies
- Aim for > 70% code coverage

**Example**:
```go
func TestGetUserByID(t *testing.T) {
    tests := []struct {
        name    string
        userID  int
        wantErr bool
    }{
        {"valid user", 1, false},
        {"user not found", 999, true},
        {"invalid ID", -1, true},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            user, err := GetUserByID(tt.userID)
            if (err != nil) != tt.wantErr {
                t.Errorf("GetUserByID() error = %v, wantErr %v", err, tt.wantErr)
            }
            // Additional assertions...
        })
    }
}
```

### Frontend Testing

- Test user interactions and UI behavior
- Test component rendering with different props
- Test API calls with mocked responses
- Test form validation and submission
- Aim for > 70% code coverage

**Example**:
```jsx
import { render, screen, fireEvent } from '@testing-library/react';
import Button from './Button';

describe('Button', () => {
  it('renders children correctly', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });

  it('calls onClick when clicked', () => {
    const handleClick = jest.fn();
    render(<Button onClick={handleClick}>Click</Button>);
    fireEvent.click(screen.getByText('Click'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });

  it('is disabled when disabled prop is true', () => {
    render(<Button disabled>Click</Button>);
    expect(screen.getByRole('button')).toBeDisabled();
  });
});
```

## 👁️ Code Review

### For Contributors

- Be open to feedback and suggestions
- Respond to all comments, even if just to acknowledge
- Don't take criticism personally - it's about the code, not you
- If you disagree, explain your reasoning politely
- Update your PR based on feedback

### For Reviewers

- Be respectful and constructive
- Focus on the code, not the person
- Explain **why** something should be changed
- Suggest specific improvements when possible
- Approve the PR if it meets standards, even if minor issues remain
- Use these labels:
  - 🔴 **Blocker**: Must be fixed before merge
  - 🟡 **Suggestion**: Nice to have, not required
  - 💡 **Nitpick**: Minor style/preference issue
  - ✅ **Approved**: Looks good to merge

### Review Checklist

- [ ] Code follows project conventions
- [ ] Tests added/updated and passing
- [ ] Documentation updated if needed
- [ ] No obvious bugs or security issues
- [ ] Performance considerations addressed
- [ ] Error handling is comprehensive
- [ ] Code is readable and maintainable

## ❓ Questions?

If you have questions, feel free to:

- Open an issue with the `question` label
- Ask in the PR discussion
- Contact the maintainers

## 🎉 Thank You!

Your contributions, no matter how small, make this project better. We appreciate your time and effort!

---

Happy coding! 🚀

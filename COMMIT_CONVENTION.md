# Commit Message Convention

This project follows the [Conventional Commits](https://www.conventionalcommits.org/) specification.

## Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

## Types

- **feat**: New feature for the user
- **fix**: Bug fix for the user
- **docs**: Documentation changes
- **style**: Code style changes (formatting, missing semi colons, etc)
- **refactor**: Code refactoring (no functional changes)
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **build**: Changes to build system or dependencies
- **ci**: Changes to CI/CD configuration
- **chore**: Other changes that don't modify src or test files
- **revert**: Revert a previous commit

## Examples

### Good commit messages

```bash
feat(auth): add JWT authentication system
fix(api): resolve timeout error in user endpoint
docs: update installation instructions in README
refactor(database): optimize query performance
test(auth): add unit tests for login flow
```

### Bad commit messages

```bash
Update files  # Too vague
Fixed stuff   # Not descriptive
WIP          # Work in progress, not acceptable
```

## Scope (optional)

The scope provides additional contextual information:
- **auth**: Authentication related
- **api**: API endpoints
- **ui**: User interface
- **database**: Database operations
- **config**: Configuration files

## Subject

- Use imperative, present tense: "add" not "added" nor "adds"
- Don't capitalize first letter
- No period (.) at the end
- Maximum 72 characters

## Body (optional)

- Explain what and why vs. how
- Wrap at 72 characters
- Separate from subject with blank line

## Footer (optional)

- Reference issue tracker IDs
- Breaking changes should start with `BREAKING CHANGE:`

## Full Example

```bash
feat(api): add user profile endpoint

Implement GET /api/users/:id endpoint to retrieve user profiles.
Includes validation middleware and error handling.

Closes #123
```

## Validation

Commits are validated by:
- **GitHub Actions**: Validates all commits in pull requests

Pull requests with invalid commit messages will be blocked.

# Time Manager Frontend

Professional React + TypeScript + Vite frontend for the Time Manager application.

## Features

- ✅ React 18+ with TypeScript
- ✅ Vite for fast development and building
- ✅ Tailwind CSS for styling
- ✅ shadcn/ui component library
- ✅ React Router v6 for routing
- ✅ Axios for HTTP requests with automatic snake_case ↔ camelCase transformation
- ✅ Authentication with JWT tokens
- ✅ Role-based access control (Employee/Manager)
- ✅ Toast notifications with Sonner
- ✅ Professional folder structure for scalability

## Architecture

```
src/
├── api/              # API layer with axios client and endpoints
├── components/       # Reusable components
│   ├── shared/      # Shared components (Header, etc.)
│   └── ui/          # shadcn/ui components
├── context/         # React contexts (AuthContext)
├── hooks/           # Custom React hooks
├── pages/           # Page components organized by role
│   ├── auth/       # Authentication pages
│   ├── employee/   # Employee pages
│   └── manager/    # Manager pages
├── routes/          # Routing configuration
│   ├── config.ts   # Centralized route configuration
│   ├── index.tsx   # Route setup with lazy loading
│   └── ProtectedRoute.tsx  # Route protection wrapper
├── types/           # TypeScript types
│   ├── api.ts      # Backend types (snake_case)
│   ├── errors.ts   # Error types
│   ├── models.ts   # Frontend types (camelCase)
│   └── index.ts    # Type exports
├── utils/           # Utility functions
│   ├── transformers.ts  # snake_case ↔ camelCase transformers
│   ├── errorHandler.ts  # Error handling utilities
│   └── jwt.ts       # JWT validation utilities
├── test/            # Test configuration
│   └── setup.ts    # Vitest setup
└── lib/             # Third-party library configuration
```

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- Backend API running on `http://localhost:8080`

### Installation

```bash
cd front
npm install
```

### Configuration

Copy `.env.example` to `.env` and configure:

```bash
VITE_API_URL=http://localhost:8080/api
```

### Development

```bash
npm run dev
```

The application will be available at `http://localhost:5173`

### Building

```bash
npm run build
```

### Preview Production Build

```bash
npm run preview
```

### Quality Checks

Run all quality checks (linting, type-checking, tests):

```bash
npm run quality
```

Or run individual checks:

```bash
npm run lint          # ESLint
npm run type-check    # TypeScript compiler
npm run test:run      # Vitest tests
npm run format        # Prettier formatting
npm run format:check  # Check formatting
```

## Key Architectural Decisions

### snake_case ↔ camelCase Transformation

The backend (Go) uses `snake_case` for JSON fields, while the frontend (TypeScript) uses `camelCase` as per conventions. We handle this automatically at the API boundary:

- **Request transformation**: camelCase → snake_case (in axios request interceptor)
- **Response transformation**: snake_case → camelCase (in axios response interceptor)
- **Type safety**: Separate types for API (`types/api.ts`) and domain (`types/models.ts`)

Example:
```typescript
// Backend returns:
{ first_name: "John", last_name: "Doe" }

// Frontend receives:
{ firstName: "John", lastName: "Doe" }
```

### Authentication Flow

1. User submits login form
2. `AuthContext.login()` calls `authApi.login()`
3. Backend returns `{token, user}` with snake_case fields
4. Response interceptor transforms to camelCase
5. Token and user stored in localStorage and React state
6. Request interceptor adds `Bearer {token}` to all subsequent requests
7. On 401 response, user is automatically logged out

### Centralized Route Configuration

Routes are centrally configured in `routes/config.ts` with lazy loading for optimal performance:

```typescript
// routes/config.ts
export const ROUTE_PATHS = {
  ROOT: '/',
  LOGIN: '/login',
  EMPLOYEE_DASHBOARD: '/employee/dashboard',
  MANAGER_DASHBOARD: '/manager/dashboard',
} as const;

export const ROUTES: RouteConfig[] = [
  {
    path: ROUTE_PATHS.LOGIN,
    element: LoginPage,
    isPublic: true,
  },
  {
    path: ROUTE_PATHS.EMPLOYEE_DASHBOARD,
    element: EmployeeDashboardPage,
    allowedRoles: ['employee'],
  },
  // ...
];
```

Benefits:
- **Type-safe paths**: No hardcoded strings throughout the app
- **Lazy loading**: Pages loaded on demand for better performance
- **Centralized configuration**: Single source of truth for all routes
- **Role-based access**: Declarative role restrictions

The `ProtectedRoute` wrapper enforces:
- Unauthenticated users → redirected to `/login`
- Wrong role (e.g., employee accessing manager route) → redirected to appropriate dashboard

## Testing

The application has comprehensive test coverage using **Vitest** and **React Testing Library**.

### Test Status

- **89/98 tests passing** (91% pass rate)
- **Unit tests**: jwt utilities, error handlers, transformers
- **Integration tests**: AuthContext, LoginPage, ProtectedRoute (partial)
- **Test commands**:
  ```bash
  npm run test            # Run tests in watch mode
  npm run test:run        # Run tests once
  npm run test:ui         # Open Vitest UI
  npm run test:coverage   # Generate coverage report
  ```

### Test Structure

```
src/
├── utils/
│   ├── jwt.test.ts               # JWT validation tests
│   ├── errorHandler.test.ts      # Error handling tests
│   └── transformers.test.ts      # snake_case ↔ camelCase tests
├── context/
│   └── AuthContext.test.tsx      # Authentication context tests
├── pages/
│   └── auth/
│       └── LoginPage.test.tsx    # Login page integration tests
└── routes/
    └── ProtectedRoute.test.tsx   # Route protection tests
```

See [TESTING.md](./TESTING.md) for detailed testing documentation.

## Next Steps

1. Complete E2E tests with Playwright for full user workflows
2. Implement Clock In/Out functionality
3. Implement Employee and Manager dashboards with time tracking
4. Add Teams, WorkingTimes, and Reports features
5. Visual regression testing for UI components

## Tech Stack

- **React** 18+ - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **Tailwind CSS** - Styling
- **shadcn/ui** - Component library
- **React Router** - Routing
- **Axios** - HTTP client
- **Sonner** - Toast notifications
- **Lucide React** - Icons
- **React Hook Form** + **Zod** - Form validation
- **date-fns** - Date utilities

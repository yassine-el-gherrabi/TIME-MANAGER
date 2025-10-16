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
│   └── ui/          # shadcn/ui components
├── context/         # React contexts (AuthContext)
├── hooks/           # Custom React hooks
├── pages/           # Page components organized by role
│   ├── auth/       # Authentication pages
│   ├── employee/   # Employee pages
│   └── manager/    # Manager pages
├── routes/          # Routing configuration
├── types/           # TypeScript types
│   ├── api.ts      # Backend types (snake_case)
│   ├── errors.ts   # Error types
│   ├── models.ts   # Frontend types (camelCase)
│   └── index.ts    # Type exports
├── utils/           # Utility functions
│   ├── transformers.ts  # snake_case ↔ camelCase transformers
│   ├── errorHandler.ts  # Error handling utilities
│   └── jwt.ts       # JWT validation utilities
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

### Protected Routes

Routes are protected using the `ProtectedRoute` wrapper:

```typescript
<Route
    path="/employee/dashboard"
element={
    <ProtectedRoute allowedRoles={['employee']}>
    <EmployeeDashboardPage />
    </ProtectedRoute>
}
/>
```

Role-based access control ensures:
- Unauthenticated users → redirected to `/login`
- Wrong role (e.g., employee accessing manager route) → redirected to appropriate dashboard

## Testing

Unit tests and E2E tests will be added in the next iteration.

- **Unit tests**: Vitest + React Testing Library
- **E2E tests**: Playwright

## Next Steps

1. Add unit tests for transformers, hooks, and components
2. Add E2E tests for authentication flow and protected routes
3. Implement Clock In/Out functionality
4. Implement Employee and Manager dashboards
5. Add Teams, WorkingTimes, and Reports features

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

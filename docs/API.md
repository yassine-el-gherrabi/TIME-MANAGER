# API Documentation

> **Status**: RESTful API specification - Technology-agnostic design
> **Last Updated**: 2025-10-06
> **Version**: 1.0
> **Base URL**: `/api`

---

## Table of Contents

1. [API Overview](#api-overview)
2. [Authentication](#authentication)
3. [Error Handling](#error-handling)
4. [Rate Limiting](#rate-limiting)
5. [Endpoints](#endpoints)
   - [Authentication](#authentication-endpoints)
   - [Users](#user-endpoints)
   - [Teams](#team-endpoints)
   - [Clocks](#clock-endpoints)
   - [Reports](#report-endpoints)
6. [Data Models](#data-models)
7. [Pagination](#pagination)
8. [Versioning](#versioning)

---

## API Overview

### Design Principles

**RESTful Architecture:**
- Resource-based URLs (`/users`, `/teams`, `/clocks`)
- HTTP methods represent actions (GET, POST, PUT, DELETE)
- Stateless requests (no server-side session)
- Standard HTTP status codes
- JSON request/response format

**API Characteristics:**
- **Protocol**: HTTP/HTTPS
- **Data Format**: JSON
- **Authentication**: JWT (JSON Web Tokens)
- **Encoding**: UTF-8
- **CORS**: Configurable (restricted to frontend domain)

### Request Format

**Headers:**
```http
Content-Type: application/json
Authorization: Bearer {access_token}
Accept: application/json
```

**Request Body (POST/PUT):**
```json
{
  "field1": "value1",
  "field2": "value2"
}
```

### Response Format

**Success Response:**
```json
{
  "data": {
    "id": 1,
    "attribute": "value"
  },
  "message": "Operation successful"
}
```

**Error Response:**
```json
{
  "error": "Error message",
  "details": {
    "field": ["Validation error message"]
  },
  "code": "ERROR_CODE"
}
```

---

## Authentication

### JWT Token Structure

**Access Token (Short-lived):**
```json
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "sub": "user_id",
    "email": "user@example.com",
    "role": "employee",
    "exp": 1696612800,
    "iat": 1696611900
  }
}
```

**Token Lifetime:**
- Access Token: 15-30 minutes
- Refresh Token: 7-30 days

### Authorization Header

**Format:**
```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Usage:**
```javascript
// Request example
fetch('/api/users', {
  headers: {
    'Authorization': `Bearer ${accessToken}`,
    'Content-Type': 'application/json'
  }
})
```

### Token Refresh Flow

```
1. Access token expires
   ↓
2. Client receives 401 Unauthorized
   ↓
3. Client sends refresh token to /auth/refresh
   ↓
4. Server validates refresh token
   ↓
5. Server issues new access token + refresh token
   ↓
6. Client retries original request with new access token
```

---

## Error Handling

### HTTP Status Codes

| Code | Meaning | Usage |
|------|---------|-------|
| 200 | OK | Successful GET, PUT request |
| 201 | Created | Successful POST request |
| 204 | No Content | Successful DELETE request |
| 400 | Bad Request | Invalid request format/parameters |
| 401 | Unauthorized | Missing or invalid authentication token |
| 403 | Forbidden | Valid token but insufficient permissions |
| 404 | Not Found | Resource doesn't exist |
| 422 | Unprocessable Entity | Validation error |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |

### Error Response Structure

**Validation Error (422):**
```json
{
  "error": "Validation failed",
  "details": {
    "email": ["Email is required", "Email must be valid"],
    "password": ["Password must be at least 8 characters"]
  },
  "code": "VALIDATION_ERROR"
}
```

**Authentication Error (401):**
```json
{
  "error": "Unauthorized",
  "message": "Invalid or expired token",
  "code": "INVALID_TOKEN"
}
```

**Authorization Error (403):**
```json
{
  "error": "Forbidden",
  "message": "You don't have permission to access this resource",
  "code": "INSUFFICIENT_PERMISSIONS"
}
```

**Not Found Error (404):**
```json
{
  "error": "Not Found",
  "message": "User with id 123 not found",
  "code": "RESOURCE_NOT_FOUND"
}
```

**Rate Limit Error (429):**
```json
{
  "error": "Too Many Requests",
  "message": "Rate limit exceeded. Try again in 60 seconds.",
  "retry_after": 60,
  "code": "RATE_LIMIT_EXCEEDED"
}
```

---

## Rate Limiting

### Limits

**Global Limits:**
- API requests: 100 requests per minute per IP
- Authentication: 5 attempts per 15 minutes per IP

**Response Headers:**
```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1696612800
```

**Exceeded Limit:**
```http
HTTP/1.1 429 Too Many Requests
Retry-After: 60

{
  "error": "Rate limit exceeded",
  "retry_after": 60
}
```

---

## Endpoints

### Authentication Endpoints

#### POST /auth/login

**Description**: Authenticate user and receive access tokens

**Authentication**: None (public endpoint)

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Success Response (200):**
```json
{
  "data": {
    "user": {
      "id": 1,
      "email": "user@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "role": "employee"
    },
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 900
  },
  "message": "Login successful"
}
```

**Error Responses:**
- `400`: Missing email or password
- `401`: Invalid credentials
- `429`: Too many login attempts

---

#### POST /auth/refresh

**Description**: Refresh access token using refresh token

**Authentication**: None (requires refresh token in body)

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Success Response (200):**
```json
{
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 900
  },
  "message": "Token refreshed"
}
```

**Error Responses:**
- `400`: Missing refresh token
- `401`: Invalid or expired refresh token

---

#### POST /auth/logout

**Description**: Logout user and invalidate tokens

**Authentication**: Required (Bearer token)

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Success Response (200):**
```json
{
  "message": "Logout successful"
}
```

---

### User Endpoints

#### GET /users

**Description**: Get list of users (paginated)

**Authentication**: Required

**Authorization**:
- Employees: Can only see themselves
- Managers: Can see all users

**Query Parameters:**
```
?page=1           # Page number (default: 1)
&limit=20         # Items per page (default: 20, max: 100)
&role=employee    # Filter by role (optional)
&search=john      # Search by name/email (optional)
```

**Success Response (200):**
```json
{
  "data": [
    {
      "id": 1,
      "email": "john@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "phone_number": "+33612345678",
      "role": "employee",
      "created_at": "2025-01-01T00:00:00Z"
    },
    {
      "id": 2,
      "email": "jane@example.com",
      "first_name": "Jane",
      "last_name": "Smith",
      "phone_number": "+33698765432",
      "role": "manager",
      "created_at": "2025-01-02T00:00:00Z"
    }
  ],
  "meta": {
    "current_page": 1,
    "per_page": 20,
    "total_pages": 5,
    "total_count": 95
  }
}
```

---

#### POST /users

**Description**: Create new user

**Authentication**: Required

**Authorization**: Manager only

**Request Body:**
```json
{
  "email": "newuser@example.com",
  "password": "password123",
  "first_name": "Alice",
  "last_name": "Johnson",
  "phone_number": "+33656781234",
  "role": "employee"
}
```

**Success Response (201):**
```json
{
  "data": {
    "id": 3,
    "email": "newuser@example.com",
    "first_name": "Alice",
    "last_name": "Johnson",
    "phone_number": "+33656781234",
    "role": "employee",
    "created_at": "2025-10-06T12:00:00Z"
  },
  "message": "User created successfully"
}
```

**Error Responses:**
- `400`: Invalid request format
- `403`: Insufficient permissions (not manager)
- `422`: Validation error (email already exists, weak password)

---

#### GET /users/:id

**Description**: Get user by ID

**Authentication**: Required

**Authorization**:
- Users can view their own profile
- Managers can view any user

**Success Response (200):**
```json
{
  "data": {
    "id": 1,
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "phone_number": "+33612345678",
    "role": "employee",
    "teams": [
      {
        "id": 1,
        "name": "Engineering",
        "joined_at": "2025-01-01T00:00:00Z"
      }
    ],
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-10-06T12:00:00Z"
  }
}
```

**Error Responses:**
- `403`: Cannot view other user's profile (not manager)
- `404`: User not found

---

#### PUT /users/:id

**Description**: Update user

**Authentication**: Required

**Authorization**:
- Users can update their own profile (except role)
- Managers can update any user

**Request Body:**
```json
{
  "first_name": "John",
  "last_name": "Doe",
  "phone_number": "+33612345678",
  "email": "newemail@example.com"
}
```

**Success Response (200):**
```json
{
  "data": {
    "id": 1,
    "email": "newemail@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "phone_number": "+33612345678",
    "role": "employee",
    "updated_at": "2025-10-06T12:30:00Z"
  },
  "message": "User updated successfully"
}
```

**Error Responses:**
- `403`: Cannot update other user (not manager)
- `404`: User not found
- `422`: Validation error (email already taken)

---

#### DELETE /users/:id

**Description**: Delete user (soft delete)

**Authentication**: Required

**Authorization**:
- Users can delete their own account
- Managers can delete any user

**Success Response (204):**
```
No content
```

**Error Responses:**
- `403`: Cannot delete other user (not manager)
- `404`: User not found

---

### Team Endpoints

#### GET /teams

**Description**: Get list of teams

**Authentication**: Required

**Success Response (200):**
```json
{
  "data": [
    {
      "id": 1,
      "name": "Engineering",
      "description": "Software development team",
      "manager": {
        "id": 2,
        "first_name": "Jane",
        "last_name": "Smith"
      },
      "member_count": 5,
      "created_at": "2025-01-01T00:00:00Z"
    }
  ]
}
```

---

#### POST /teams

**Description**: Create new team

**Authentication**: Required

**Authorization**: Manager only

**Request Body:**
```json
{
  "name": "Marketing",
  "description": "Marketing and communications team",
  "manager_id": 2
}
```

**Success Response (201):**
```json
{
  "data": {
    "id": 2,
    "name": "Marketing",
    "description": "Marketing and communications team",
    "manager": {
      "id": 2,
      "first_name": "Jane",
      "last_name": "Smith"
    },
    "created_at": "2025-10-06T12:00:00Z"
  },
  "message": "Team created successfully"
}
```

**Error Responses:**
- `403`: Insufficient permissions (not manager)
- `422`: Validation error (name already exists, manager_id invalid)

---

#### GET /teams/:id

**Description**: Get team details with members

**Authentication**: Required

**Success Response (200):**
```json
{
  "data": {
    "id": 1,
    "name": "Engineering",
    "description": "Software development team",
    "manager": {
      "id": 2,
      "first_name": "Jane",
      "last_name": "Smith",
      "email": "jane@example.com"
    },
    "members": [
      {
        "id": 1,
        "first_name": "John",
        "last_name": "Doe",
        "email": "john@example.com",
        "role": "employee",
        "joined_at": "2025-01-01T00:00:00Z"
      },
      {
        "id": 3,
        "first_name": "Alice",
        "last_name": "Johnson",
        "email": "alice@example.com",
        "role": "employee",
        "joined_at": "2025-01-05T00:00:00Z"
      }
    ],
    "created_at": "2025-01-01T00:00:00Z",
    "updated_at": "2025-10-06T12:00:00Z"
  }
}
```

---

#### PUT /teams/:id

**Description**: Update team

**Authentication**: Required

**Authorization**: Manager only (team manager or admin)

**Request Body:**
```json
{
  "name": "Engineering Team",
  "description": "Updated description",
  "manager_id": 2
}
```

**Success Response (200):**
```json
{
  "data": {
    "id": 1,
    "name": "Engineering Team",
    "description": "Updated description",
    "manager": {
      "id": 2,
      "first_name": "Jane",
      "last_name": "Smith"
    },
    "updated_at": "2025-10-06T12:30:00Z"
  },
  "message": "Team updated successfully"
}
```

---

#### DELETE /teams/:id

**Description**: Delete team (soft delete)

**Authentication**: Required

**Authorization**: Manager only

**Success Response (204):**
```
No content
```

---

#### POST /teams/:id/members

**Description**: Add member to team

**Authentication**: Required

**Authorization**: Manager only (team manager)

**Request Body:**
```json
{
  "user_id": 5
}
```

**Success Response (201):**
```json
{
  "data": {
    "user": {
      "id": 5,
      "first_name": "Bob",
      "last_name": "Wilson",
      "email": "bob@example.com"
    },
    "joined_at": "2025-10-06T12:00:00Z"
  },
  "message": "Member added to team"
}
```

**Error Responses:**
- `403`: Insufficient permissions
- `404`: Team or user not found
- `422`: User already in team

---

#### DELETE /teams/:id/members/:user_id

**Description**: Remove member from team

**Authentication**: Required

**Authorization**: Manager only (team manager)

**Success Response (204):**
```
No content
```

---

### Clock Endpoints

#### POST /clocks

**Description**: Clock in or clock out (toggles status)

**Authentication**: Required

**Request Body:** (Optional - uses authenticated user)
```json
{
  "time": "2025-10-06T08:30:00Z"  // Optional, defaults to current time
}
```

**Success Response (201):**
```json
{
  "data": {
    "id": 123,
    "user_id": 1,
    "time": "2025-10-06T08:30:00Z",
    "status": "arrival",
    "created_at": "2025-10-06T08:30:00Z"
  },
  "message": "Clocked in successfully"
}
```

**Business Logic:**
- If last clock was "departure" or no clock exists → create "arrival"
- If last clock was "arrival" → create "departure"

**Error Responses:**
- `422`: Cannot clock in (validation error, future time, etc.)

---

#### GET /users/:id/clocks

**Description**: Get user's clock history

**Authentication**: Required

**Authorization**:
- Users can view their own clocks
- Managers can view team members' clocks

**Query Parameters:**
```
?start_date=2025-10-01  # Filter by start date (optional)
&end_date=2025-10-31    # Filter by end date (optional)
&page=1                 # Page number
&limit=50               # Items per page
```

**Success Response (200):**
```json
{
  "data": [
    {
      "id": 123,
      "time": "2025-10-06T08:30:00Z",
      "status": "arrival"
    },
    {
      "id": 124,
      "time": "2025-10-06T17:00:00Z",
      "status": "departure"
    }
  ],
  "meta": {
    "current_page": 1,
    "per_page": 50,
    "total_count": 120
  }
}
```

---

#### GET /users/:id/working-hours

**Description**: Get calculated working hours

**Authentication**: Required

**Authorization**:
- Users can view their own hours
- Managers can view team members' hours

**Query Parameters:**
```
?start_date=2025-10-01  # Required
&end_date=2025-10-31    # Required
&group_by=day           # day|week|month (default: day)
```

**Success Response (200):**
```json
{
  "data": {
    "summary": {
      "total_hours": 168.5,
      "average_daily_hours": 7.5,
      "working_days": 22
    },
    "breakdown": [
      {
        "date": "2025-10-01",
        "hours": 8.5,
        "arrival": "2025-10-01T08:30:00Z",
        "departure": "2025-10-01T17:00:00Z"
      },
      {
        "date": "2025-10-02",
        "hours": 8.0,
        "arrival": "2025-10-02T09:00:00Z",
        "departure": "2025-10-02T17:00:00Z"
      }
    ]
  }
}
```

---

### Report Endpoints

#### GET /reports

**Description**: Get global KPI reports

**Authentication**: Required

**Authorization**: Manager only

**Query Parameters:**
```
?start_date=2025-10-01  # Required
&end_date=2025-10-31    # Required
&team_id=1              # Filter by team (optional)
&kpi=lateness           # Specific KPI (optional: lateness, hours)
```

**Success Response (200):**
```json
{
  "data": {
    "period": {
      "start": "2025-10-01",
      "end": "2025-10-31"
    },
    "kpis": {
      "lateness_rate": {
        "value": 12.5,
        "unit": "percentage",
        "description": "Percentage of arrivals after 9:00 AM",
        "trend": "increasing"
      },
      "average_weekly_hours": {
        "value": 38.2,
        "unit": "hours",
        "description": "Average weekly working hours",
        "trend": "stable"
      },
      "total_employees": 25,
      "active_employees": 23
    },
    "top_performers": [
      {
        "user_id": 5,
        "name": "Alice Johnson",
        "hours": 42.5,
        "lateness_count": 0
      }
    ],
    "teams": [
      {
        "team_id": 1,
        "name": "Engineering",
        "average_hours": 39.5,
        "lateness_rate": 8.3
      }
    ]
  }
}
```

---

#### GET /teams/:id/reports

**Description**: Get team-specific reports

**Authentication**: Required

**Authorization**: Team manager only

**Query Parameters:**
```
?start_date=2025-10-01
&end_date=2025-10-31
```

**Success Response (200):**
```json
{
  "data": {
    "team": {
      "id": 1,
      "name": "Engineering"
    },
    "period": {
      "start": "2025-10-01",
      "end": "2025-10-31"
    },
    "summary": {
      "total_members": 5,
      "average_daily_hours": 7.8,
      "average_weekly_hours": 39.0,
      "lateness_rate": 8.3
    },
    "members": [
      {
        "user_id": 1,
        "name": "John Doe",
        "total_hours": 168.5,
        "average_daily_hours": 7.7,
        "late_arrivals": 2
      },
      {
        "user_id": 3,
        "name": "Alice Johnson",
        "total_hours": 176.0,
        "average_daily_hours": 8.0,
        "late_arrivals": 0
      }
    ]
  }
}
```

---

## Data Models

### User Model

```json
{
  "id": 1,
  "email": "user@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "phone_number": "+33612345678",
  "role": "employee",
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-10-06T12:00:00Z"
}
```

**Note**: `password_hash` is never returned in responses

### Team Model

```json
{
  "id": 1,
  "name": "Engineering",
  "description": "Software development team",
  "manager_id": 2,
  "created_at": "2025-01-01T00:00:00Z",
  "updated_at": "2025-10-06T12:00:00Z"
}
```

### Clock Model

```json
{
  "id": 123,
  "user_id": 1,
  "time": "2025-10-06T08:30:00Z",
  "status": "arrival",
  "created_at": "2025-10-06T08:30:00Z"
}
```

**Status Values**: `"arrival"` | `"departure"`

---

## Pagination

### Request

**Query Parameters:**
```
?page=1       # Page number (1-indexed)
&limit=20     # Items per page (max: 100)
```

### Response

**Meta Object:**
```json
{
  "meta": {
    "current_page": 1,
    "per_page": 20,
    "total_pages": 5,
    "total_count": 95
  }
}
```

---

## Versioning

### URL Versioning (Recommended for Future)

```
Current: /api/users
Future:  /api/v2/users
```

**Deprecation Policy:**
- Minimum 6 months notice before deprecation
- Support previous version for 12 months
- Clear migration guide provided

---

**Document Status**: API specification - Update as endpoints are implemented
**Review Frequency**: After each sprint, before major releases
**Owner**: Backend Team

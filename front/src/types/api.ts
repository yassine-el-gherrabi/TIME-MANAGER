// Backend API Response Types (snake_case)
// These types match the exact format returned by the Go backend

export interface UserResponse {
  id: number;
  email: string;
  first_name: string;
  last_name: string;
  phone_number?: string;
  role: 'employee' | 'manager';
  team_id?: number;
  created_at: string;
  updated_at: string;
}

export interface TeamResponse {
  id: number;
  name: string;
  description?: string;
  manager_id: number;
  member_ids: number[];
  created_at: string;
  updated_at: string;
}

export interface ClockResponse {
  id: number;
  user_id: number;
  timestamp: string;
  status: 'clock_in' | 'clock_out';
  created_at: string;
}

export interface WorkingTimeResponse {
  id: number;
  user_id: number;
  start_time: string;
  end_time: string;
  duration: number;
  date: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: UserResponse;
}

export interface ApiError {
  error: string;
  message?: string;
}

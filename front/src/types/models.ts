// Frontend Domain Types (camelCase)
// These types are used throughout the frontend application

export interface User {
  id: number;
  email: string;
  firstName: string;
  lastName: string;
  phoneNumber?: string;
  role: 'employee' | 'manager';
  teamId?: number;
  team?: Team;
  createdAt: string;
  updatedAt: string;
}

export interface Team {
  id: number;
  name: string;
  description?: string;
  managerId: number;
  manager?: User;
  members?: User[];
  memberIds: number[];
  createdAt: string;
  updatedAt: string;
}

export interface Clock {
  id: number;
  userId: number;
  timestamp: string;
  status: 'clock_in' | 'clock_out';
  createdAt: string;
}

export interface WorkingTime {
  id: number;
  userId: number;
  user?: User;
  startTime: string;
  endTime: string;
  duration: number;
  date: string;
}

export interface DailyStats {
  date: string;
  duration: number;
  clockIn?: string;
  clockOut?: string;
}

export interface UserStatistics {
  userId: number;
  today: number;
  thisWeek: number;
  thisMonth: number;
  averageWeekly: number;
  punctualityRate: number;
  totalDays: number;
  onTimeDays: number;
  lateDays: number;
  last7Days: DailyStats[];
}

export interface Alert {
  userId: number;
  userName: string;
  type: 'absence' | 'late' | 'undertime';
  message: string;
  severity: 'low' | 'medium' | 'high';
}

export interface TeamKPI {
  teamId: number;
  presenceRate: number;
  punctualityRate: number;
  averageHoursPerEmployee: number;
  overtimeHours: number;
  alerts: Alert[];
  trends: {
    last90Days: number[];
  };
}

export interface CurrentStatus {
  isClocked: boolean;
  lastClock?: Clock;
  currentDuration: number;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface AuthState {
  token: string;
  user: User;
}

export interface TeamReport {
  teamId: number;
  teamName: string;
  startDate: string;
  endDate: string;
  totalHours: number;
  averageHoursPerMember: number;
  members: Array<{
    userId: number;
    userName: string;
    totalHours: number;
    daysWorked: number;
  }>;
}

export interface UserReport {
  userId: number;
  userName: string;
  startDate: string;
  endDate: string;
  totalHours: number;
  daysWorked: number;
  averageHoursPerDay: number;
  sessions: WorkingTime[];
}

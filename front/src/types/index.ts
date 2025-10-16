// Re-export all types for convenient imports
export * from './models';

// Common utility types
export type Role = 'employee' | 'manager';
export type ClockStatus = 'clock_in' | 'clock_out';
export type AlertType = 'absence' | 'late' | 'undertime';
export type AlertSeverity = 'low' | 'medium' | 'high';

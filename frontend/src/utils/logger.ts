/**
 * Structured Logger Service
 *
 * Centralized logging with log levels, context support, and environment awareness.
 * Replaces raw console.* calls for better debugging and maintainability.
 */

type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LogContext {
  component?: string;
  action?: string;
  userId?: string;
  [key: string]: unknown;
}

interface LogEntry {
  level: LogLevel;
  message: string;
  timestamp: string;
  context?: LogContext;
  error?: {
    name: string;
    message: string;
    stack?: string;
  };
}

const LOG_LEVELS: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

// In production, only show warnings and errors
const MIN_LOG_LEVEL: LogLevel = import.meta.env.DEV ? 'debug' : 'warn';

/**
 * Serialize an error object for logging
 */
function serializeError(error: unknown): LogEntry['error'] | undefined {
  if (!error) return undefined;

  if (error instanceof Error) {
    return {
      name: error.name,
      message: error.message,
      stack: import.meta.env.DEV ? error.stack : undefined,
    };
  }

  // Handle non-Error objects
  return {
    name: 'UnknownError',
    message: String(error),
  };
}

/**
 * Format log entry for console output
 */
function formatLogEntry(entry: LogEntry): string {
  const parts = [`[${entry.level.toUpperCase()}]`, entry.message];

  if (entry.context?.component) {
    parts.unshift(`[${entry.context.component}]`);
  }

  return parts.join(' ');
}

/**
 * Check if a log level should be displayed
 */
function shouldLog(level: LogLevel): boolean {
  return LOG_LEVELS[level] >= LOG_LEVELS[MIN_LOG_LEVEL];
}

/**
 * Core logging function
 */
function log(level: LogLevel, message: string, error?: unknown, context?: LogContext): void {
  if (!shouldLog(level)) return;

  const entry: LogEntry = {
    level,
    message,
    timestamp: new Date().toISOString(),
    context,
    error: serializeError(error),
  };

  const formattedMessage = formatLogEntry(entry);

  // Use appropriate console method
  switch (level) {
    case 'debug':
      if (import.meta.env.DEV) {
        console.debug(formattedMessage, context ? { context } : '', error || '');
      }
      break;
    case 'info':
      console.info(formattedMessage, context ? { context } : '');
      break;
    case 'warn':
      console.warn(formattedMessage, context ? { context } : '', error || '');
      break;
    case 'error':
      console.error(formattedMessage, context ? { context } : '', error || '');
      break;
  }
}

/**
 * Logger instance with typed methods
 */
export const logger = {
  /**
   * Debug level - only shown in development
   */
  debug(message: string, context?: LogContext): void {
    log('debug', message, undefined, context);
  },

  /**
   * Info level - general information
   */
  info(message: string, context?: LogContext): void {
    log('info', message, undefined, context);
  },

  /**
   * Warning level - potential issues
   */
  warn(message: string, error?: unknown, context?: LogContext): void {
    log('warn', message, error, context);
  },

  /**
   * Error level - errors that need attention
   */
  error(message: string, error?: unknown, context?: LogContext): void {
    log('error', message, error, context);
  },
};

export type { LogLevel, LogContext, LogEntry };

// Application Constants Configuration
export const APP_CONFIG = {
  APP_NAME: 'Time Manager',
  APP_VERSION: '1.0.0',
  APP_DESCRIPTION: 'Professional Time Management Application',

  // Pagination
  PAGINATION: {
    DEFAULT_PAGE_SIZE: 10,
    PAGE_SIZE_OPTIONS: [10, 25, 50, 100],
  },

  // Date/Time Formats
  DATE_FORMATS: {
    DISPLAY: 'dd/MM/yyyy',
    DISPLAY_TIME: 'dd/MM/yyyy HH:mm',
    API: 'yyyy-MM-dd',
    API_TIME: "yyyy-MM-dd'T'HH:mm:ss",
  },

  // Local Storage Keys
  STORAGE_KEYS: {
    AUTH_TOKEN: 'authToken',
    USER_DATA: 'userData',
    THEME: 'theme',
  },

  // User Roles
  ROLES: {
    ADMIN: 'admin',
    MANAGER: 'manager',
    EMPLOYEE: 'employee',
  },
};

export default APP_CONFIG;

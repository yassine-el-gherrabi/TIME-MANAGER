// API Endpoints Configuration
export const API_ENDPOINTS = {
  // Authentication
  AUTH: {
    LOGIN: '/auth/login',
    REGISTER: '/auth/register',
    LOGOUT: '/auth/logout',
    REFRESH_TOKEN: '/auth/refresh',
    ME: '/auth/me',
  },

  // Users
  USERS: {
    LIST: '/users',
    DETAIL: (id) => `/users/${id}`,
    CREATE: '/users',
    UPDATE: (id) => `/users/${id}`,
    DELETE: (id) => `/users/${id}`,
  },

  // Working Times
  WORKING_TIMES: {
    LIST: '/workingtimes',
    DETAIL: (id) => `/workingtimes/${id}`,
    USER_WORKING_TIMES: (userId) => `/workingtimes/${userId}`,
    CREATE: '/workingtimes',
    UPDATE: (id) => `/workingtimes/${id}`,
    DELETE: (id) => `/workingtimes/${id}`,
  },

  // Clocks
  CLOCKS: {
    LIST: '/clocks',
    USER_CLOCK: (userId) => `/clocks/${userId}`,
    CREATE: '/clocks',
  },
};

export default API_ENDPOINTS;

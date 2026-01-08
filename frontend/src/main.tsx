import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import { initializeAuth } from './stores/authStore'

/**
 * Initialize application
 *
 * Awaits auth initialization before rendering to prevent
 * authenticated users from being redirected to /login on page reload.
 */
const initializeApp = async () => {
  // Wait for auth state to be restored from refresh token
  await initializeAuth();

  // Render app after auth is initialized
  ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  )
}

initializeApp();

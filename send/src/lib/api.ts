import axios from 'axios';

const api = axios.create();
const BACKEND_URL = '/api';

// Types for services
export interface Service {
  id: string;
  name: string;
  description?: string;
  rating: number;
  price?: number;
  image: string;
  location?: string;
}

/**
 * Get services by type
 */
export const getServices = async (type: string): Promise<Service[]> => {
  try {
    const response = await api.get(`${BACKEND_URL}/services?type=${encodeURIComponent(type)}`);
    return response.data || [];
  } catch (error) {
    console.error('Error fetching services:', error);
    return [];
  }
};

/**
 * Search services with query and optional filters
 */
export const searchServices = async (
  query: string, 
  filters: Record<string, string | number | boolean> = {}
): Promise<Service[]> => {
  try {
    const params = new URLSearchParams();
    params.append('q', query);
    
    Object.entries(filters).forEach(([key, value]) => {
      if (value !== undefined && value !== null && value !== '') {
        params.append(key, value.toString());
      }
    });
    
    const response = await api.get(`${BACKEND_URL}/search?${params.toString()}`);
    return response.data || [];
  } catch (error) {
    console.error('Error searching services:', error);
    return [];
  }
};

/**
 * Get a single service by ID
 */
export const getServiceById = async (id: string): Promise<Service | null> => {
  try {
    const response = await api.get(`${BACKEND_URL}/service/${encodeURIComponent(id)}`);
    return response.data;
  } catch (error) {
    console.error(`Error fetching service ${id}:`, error);
    return null;
  }
};

/**
 * Redirect to Google OAuth login
 */
export const googleAuth = () => {
  window.location.href = '/auth/google';
};

/**
 * Get the stored auth token
 */
export const getToken = () => {
  return localStorage.getItem('auth_token');
};

/**
 * Store the auth token and set it for API calls
 */
export const setToken = (token: string) => {
  localStorage.setItem('auth_token', token);
  
  // Set default auth header for all future requests
  api.defaults.headers.common['Authorization'] = `Bearer ${token}`;
};

/**
 * Clear the auth token
 */
export const clearToken = () => {
  localStorage.removeItem('auth_token');
  delete api.defaults.headers.common['Authorization'];
};

/**
 * Check if the user is authenticated
 */
export const isAuthenticated = () => {
  return !!getToken();
};

/**
 * Process auth callback - call this on page load to handle OAuth redirects
 */
export const handleAuthCallback = () => {
  const params = new URLSearchParams(window.location.search);
  const token = params.get('token');
  
  if (token) {
    setToken(token);
    // Clean URL
    window.history.replaceState({}, document.title, window.location.pathname);
    return true;
  }
  return false;
};

// Add auth token to all requests
api.interceptors.request.use(config => {
  const token = getToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
}); 
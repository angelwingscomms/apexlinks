import axios from 'axios';

const api = axios.create();

export const getServices = async (type: string) => {
  try {
    const response = await api.get(`/api/services?type=${type}`);
    return response.data;
  } catch (error) {
    console.error('Error fetching services:', error);
    return [];
  }
};

export const googleAuth = () => {
  window.location.href = '/auth/google';
};

export const getToken = () => {
  return localStorage.getItem('auth_token');
};

export const setToken = (token: string) => {
  localStorage.setItem('auth_token', token);
};

// Add auth token to all requests
api.interceptors.request.use(config => {
  const token = getToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
}); 
import { writable, derived } from 'svelte/store';
import axios from 'axios';

// API base URL
const API_URL = 'http://localhost:8000';

// User interface
export interface User {
  id: string;
  name: string;
  username?: string;
  email: string;
  picture: string;
  description?: string;
  age?: number;
  gender?: string;
  zone_id?: string;
}

// Initial state
interface UserState {
  user: User | null;
  isAuthenticated: boolean;
  loading: boolean;
  error: string | null;
}

const initialState: UserState = {
  user: null,
  isAuthenticated: false,
  loading: false,
  error: null
};

// Create the store
const createUserStore = () => {
  const { subscribe, set, update } = writable<UserState>(initialState);

  // Check for existing user in localStorage on initialization
  const initializeFromStorage = () => {
    if (typeof window !== 'undefined') {
      const storedUser = localStorage.getItem('user');
      if (storedUser) {
        try {
          const user = JSON.parse(storedUser);
          update(state => ({
            ...state,
            user,
            isAuthenticated: true
          }));
        } catch (error) {
          console.error('Failed to parse stored user data', error);
        }
      }
    }
  };

  return {
    subscribe,
    
    // Initialize the store
    initialize: () => {
      initializeFromStorage();
    },

    // Start Google OAuth flow
    startGoogleAuth: async () => {
      update(state => ({ ...state, loading: true, error: null }));
      try {
        const response = await axios.get(`${API_URL}/auth/google/login`);
        if (response.data?.auth_url) {
          window.location.href = response.data.auth_url;
        } else {
          throw new Error('No auth URL received');
        }
      } catch (error) {
        console.error('Failed to start Google auth', error);
        update(state => ({ 
          ...state, 
          loading: false, 
          error: 'Failed to start authentication' 
        }));
      }
    },

    // Handle OAuth callback
    handleAuthCallback: async (code: string, state: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      try {
        const response = await axios.get(`${API_URL}/auth/google/callback?code=${code}&state=${state}`);
        
        if (response.data?.user) {
          const user = response.data.user;
          
          // Store in localStorage
          if (typeof window !== 'undefined') {
            localStorage.setItem('user', JSON.stringify(user));
            localStorage.setItem('user_id', response.data.user_id);
          }
          
          update(state => ({
            ...state,
            user,
            isAuthenticated: true,
            loading: false,
            error: null
          }));
          
          return true;
        } else {
          throw new Error('No user data received');
        }
      } catch (error) {
        console.error('Failed to complete authentication', error);
        update(state => ({ 
          ...state, 
          loading: false, 
          error: 'Authentication failed' 
        }));
        return false;
      }
    },

    // Update user profile
    updateProfile: async (userData: Partial<User>) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        // Get current user ID
        const userId = localStorage.getItem('user_id');
        if (!userId) {
          throw new Error('User ID not found');
        }
        
        // TODO: Implement backend endpoint for profile update
        // For now, we'll just update the local state
        
        update(state => {
          if (!state.user) return state;
          
          const updatedUser = {
            ...state.user,
            ...userData
          };
          
          // Update localStorage
          if (typeof window !== 'undefined') {
            localStorage.setItem('user', JSON.stringify(updatedUser));
          }
          
          return {
            ...state,
            user: updatedUser,
            loading: false
          };
        });
        
        return true;
      } catch (error) {
        console.error('Failed to update profile', error);
        update(state => ({ 
          ...state, 
          loading: false, 
          error: 'Failed to update profile' 
        }));
        return false;
      }
    },

    // Logout user
    logout: () => {
      if (typeof window !== 'undefined') {
        localStorage.removeItem('user');
        localStorage.removeItem('user_id');
      }
      
      set(initialState);
    }
  };
};

export const userStore = createUserStore();

// Derived stores for convenience
export const isAuthenticated = derived(
  userStore,
  $userStore => $userStore.isAuthenticated
);

export const currentUser = derived(
  userStore,
  $userStore => $userStore.user
);

export const isLoading = derived(
  userStore,
  $userStore => $userStore.loading
);

export const authError = derived(
  userStore,
  $userStore => $userStore.error
); 
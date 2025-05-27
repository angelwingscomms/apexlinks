import axios from 'axios';
import type { User } from '../stores/userStore';

// API base URL
const API_URL = 'http://localhost:8000';

export interface UserSearchParams {
  query: string;
  s: string; // tenant identifier
  limit?: number;
  age?: number;
  gender?: string;
}

export interface UserSearchResult {
  id: string;
  name?: string;
  email?: string;
  description?: string;
  username?: string;
  zone_id?: string;
  score?: number;
}

export interface SimilarityResult {
  similarity_score: number;
  common_interests: string[];
  matching_attributes: Record<string, any>;
}

const userService = {
  /**
   * Search for users based on provided parameters
   */
  searchUsers: async (params: UserSearchParams): Promise<UserSearchResult[]> => {
    try {
      const response = await axios.post(`${API_URL}/user/search`, params);
      return response.data.users || [];
    } catch (error) {
      console.error('Error searching users:', error);
      throw error;
    }
  },

  /**
   * Get user by ID
   */
  getUserById: async (userId: string): Promise<User | null> => {
    try {
      // Assuming there's an endpoint to get user by ID
      // This might need to be implemented on the backend
      const response = await axios.get(`${API_URL}/user/${userId}`);
      return response.data || null;
    } catch (error) {
      console.error('Error fetching user:', error);
      throw error;
    }
  },

  /**
   * Calculate similarity between two users
   */
  calculateSimilarity: async (userId1: string, userId2: string): Promise<SimilarityResult> => {
    try {
      const response = await axios.post(`${API_URL}/user/similarity`, {
        user_id1: userId1,
        user_id2: userId2
      });
      return response.data || { similarity_score: 0, common_interests: [], matching_attributes: {} };
    } catch (error) {
      console.error('Error calculating user similarity:', error);
      throw error;
    }
  },

  /**
   * Join a zone
   */
  joinZone: async (zoneId: string): Promise<boolean> => {
    try {
      await axios.post(`${API_URL}/user/join_zone`, { zone_id: zoneId });
      return true;
    } catch (error) {
      console.error('Error joining zone:', error);
      throw error;
    }
  },

  /**
   * Leave a zone
   */
  leaveZone: async (zoneId: string): Promise<boolean> => {
    try {
      await axios.post(`${API_URL}/user/leave_zone`, { zone_id: zoneId });
      return true;
    } catch (error) {
      console.error('Error leaving zone:', error);
      throw error;
    }
  }
};

export default userService; 
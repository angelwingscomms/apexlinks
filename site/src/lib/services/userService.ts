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
  minAge?: number;
  maxAge?: number;
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
      // Convert minAge and maxAge to a format the API understands
      const apiParams: Record<string, any> = { ...params };
      
      // Handle age filters
      if (params.minAge !== undefined || params.maxAge !== undefined) {
        const ageFilters = [];
        if (params.minAge !== undefined) {
          ageFilters.push(`>=${params.minAge}`);
        }
        if (params.maxAge !== undefined) {
          ageFilters.push(`<=${params.maxAge}`);
        }
        apiParams.age_filter = ageFilters.join(',');
        
        // Remove the individual min/max fields before sending
        delete apiParams.minAge;
        delete apiParams.maxAge;
      }
      
      const response = await axios.post(`${API_URL}/user/search`, apiParams);
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
      const response = await axios.get(`${API_URL}/user/${userId}`);
      
      // Convert the API response to our User type format
      if (response.data) {
        return {
          id: response.data.id,
          name: response.data.name || '',
          email: response.data.email,
          description: response.data.description,
          username: response.data.username,
          picture: response.data.picture,
          age: response.data.age,
          gender: response.data.gender,
          zone_id: response.data.zone_id
        };
      }
      return null;
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
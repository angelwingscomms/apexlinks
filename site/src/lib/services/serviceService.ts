import axios from 'axios';

// API base URL
const API_URL = 'http://localhost:8000';

export interface Service {
  id: string;
  name: string;
  description?: string;
  rating: number;
  price?: number;
  image?: string;
  location?: string;
  type?: string;
}

export interface ServiceSearchParams {
  query?: string;
  type?: string; 
  limit?: number;
  minRating?: number;
  maxPrice?: number;
  location?: string;
}

const serviceService = {
  /**
   * Search for services based on provided parameters
   */
  searchServices: async (params: ServiceSearchParams): Promise<Service[]> => {
    try {
      const apiParams: Record<string, any> = { ...params };
      
      const response = await axios.post(`${API_URL}/service/search`, apiParams);
      return response.data.services || [];
    } catch (error) {
      console.error('Error searching services:', error);
      throw error;
    }
  },

  /**
   * Get service by ID
   */
  getServiceById: async (serviceId: string): Promise<Service | null> => {
    try {
      const response = await axios.get(`${API_URL}/service/${serviceId}`);
      
      if (response.data) {
        return response.data;
      }
      return null;
    } catch (error) {
      console.error('Error fetching service:', error);
      throw error;
    }
  },
  
  /**
   * Get services by type
   */
  getServicesByType: async (type: string): Promise<Service[]> => {
    try {
      const response = await axios.get(`${API_URL}/services?type=${encodeURIComponent(type)}`);
      return response.data || [];
    } catch (error) {
      console.error('Error fetching services by type:', error);
      return [];
    }
  }
};

export default serviceService; 
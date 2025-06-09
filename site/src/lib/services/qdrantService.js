import axios from 'axios';

const QDRANT_API_URL = import.meta.env.VITE_QDRANT_API_URL || 'http://localhost:6333';
const QDRANT_API_KEY = import.meta.env.VITE_QDRANT_API_KEY;

// Type definitions
interface SearchFilter {
  minPrice?: number;
  maxPrice?: number;
  category?: string;
  type?: string;
  [key: string]: any;
}

interface Location {
  lat: number;
  lon: number;
}

interface SearchOptions {
  limit?: number;
  threshold?: number;
  filter?: SearchFilter;
  location?: Location;
}

interface SearchResult {
  id: string;
  payload: {
    name?: string;
    description?: string;
    imageUrl?: string;
    location?: Location;
    [key: string]: any;
  };
  score: number;
  distance?: number;
}

/**
 * Service for interacting with Qdrant vector database
 */
export const qdrantService = {
  /**
   * Search for similar items based on embedding vector
   * @param embedding - Vector embedding of search query
   * @param options - Search options
   * @returns Search results
   */
  async searchSimilarItems(embedding: number[], options: SearchOptions = {}): Promise<SearchResult[]> {
    const { limit = 10, threshold = 0.72, filter = {} } = options;
    const location = options.location || null;
    
    // Build filter conditions
    const mustConditions = [];
    
    // Add price range filter if provided
    if (filter.minPrice !== undefined && filter.maxPrice !== undefined) {
      mustConditions.push({
        range: {
          price: {
            gte: filter.minPrice,
            lte: filter.maxPrice
          }
        }
      });
    }
    
    // Add category filter if provided
    if (filter.category) {
      mustConditions.push({
        key: 'category',
        match: { value: filter.category }
      });
    }
    
    // Add type filter (product/service) if provided
    if (filter.type) {
      mustConditions.push({
        key: 'type',
        match: { value: filter.type }
      });
    }
    
    // Build search request
    const searchRequest: any = {
      vector: embedding,
      limit,
      with_payload: true,
      params: {
        hnsw_ef: 128,
        exact: false
      }
    };
    
    // Add filter if we have conditions
    if (mustConditions.length > 0) {
      searchRequest.filter = {
        must: mustConditions
      };
    }
    
    // Add score threshold
    searchRequest.score_threshold = threshold;
    
    try {
      const headers: Record<string, string> = {};
      if (QDRANT_API_KEY) {
        headers['api-key'] = QDRANT_API_KEY;
      }
      
      // Call Qdrant search endpoint
      const response = await axios.post(
        `${QDRANT_API_URL}/collections/items/points/search`,
        searchRequest,
        { headers }
      );
      
      // Process and sort results
      let results: SearchResult[] = response.data.result || [];
      
      // If location is provided, calculate distance and re-sort
      if (location && location.lat && location.lon) {
        results = results.map(result => {
          // Calculate distance if item has location
          if (result.payload.location && 
              result.payload.location.lat && 
              result.payload.location.lon) {
            result.distance = calculateDistance(
              location.lat, location.lon,
              result.payload.location.lat, result.payload.location.lon
            );
          } else {
            result.distance = Number.MAX_VALUE;
          }
          return result;
        });
        
        // Sort by distance
        results.sort((a, b) => (a.distance || 0) - (b.distance || 0));
      }
      
      return results;
    } catch (error) {
      console.error('Error searching Qdrant:', error);
      throw error;
    }
  }
};

/**
 * Calculate distance between two points using Haversine formula
 */
function calculateDistance(lat1: number, lon1: number, lat2: number, lon2: number): number {
  const R = 6371; // Earth's radius in km
  const dLat = (lat2 - lat1) * Math.PI / 180;
  const dLon = (lon2 - lon1) * Math.PI / 180;
  
  const a = 
    Math.sin(dLat/2) * Math.sin(dLat/2) +
    Math.cos(lat1 * Math.PI / 180) * Math.cos(lat2 * Math.PI / 180) * 
    Math.sin(dLon/2) * Math.sin(dLon/2);
  
  const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1-a));
  return R * c;
} 
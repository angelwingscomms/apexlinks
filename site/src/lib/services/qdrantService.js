import axios from 'axios';

const QDRANT_API_URL = import.meta.env.VITE_QDRANT_API_URL || 'http://localhost:6333';
const QDRANT_API_KEY = import.meta.env.VITE_QDRANT_API_KEY;

/**
 * Service for interacting with Qdrant vector database
 */
export const qdrantService = {
  /**
   * Search for similar items based on embedding vector
   * @param {number[]} embedding - Vector embedding of search query
   * @param {object} options - Search options
   * @param {number} options.limit - Max number of results
   * @param {number} options.threshold - Similarity threshold (0-1)
   * @param {object} options.filter - Additional filters (price range, category, etc)
   * @param {object} options.location - User location for geo filtering
   * @returns {Promise<Array>} - Search results
   */
  async searchSimilarItems(embedding, options = {}) {
    const { limit = 10, threshold = 0.72, filter = {}, location = null } = options;
    
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
    const searchRequest = {
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
      const headers = {};
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
      let results = response.data.result || [];
      
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
        results.sort((a, b) => a.distance - b.distance);
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
 * @param {number} lat1 - Latitude of first point
 * @param {number} lon1 - Longitude of first point
 * @param {number} lat2 - Latitude of second point
 * @param {number} lon2 - Longitude of second point
 * @returns {number} - Distance in kilometers
 */
function calculateDistance(lat1, lon1, lat2, lon2) {
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
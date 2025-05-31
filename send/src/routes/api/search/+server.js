import { json } from '@sveltejs/kit';
import axios from 'axios';

// Mock data for demonstration - in production, this would be retrieved from a database
const services = [
  {
    id: 'mech1',
    name: 'Quick Fix Auto',
    description: 'Fast and reliable car repair services for all makes and models',
    rating: 4.7,
    price: 80,
    image: '/images/mechanic1.jpg',
    location: '2.5 miles away',
    type: 'mechanics',
    vectors: [0.1, 0.2, 0.3]  // Mock embedding
  },
  {
    id: 'mech2',
    name: 'Elite Car Care',
    description: 'Premium auto repair with certified technicians',
    rating: 4.9,
    price: 120,
    image: '/images/mechanic2.jpg',
    location: '3.8 miles away',
    type: 'mechanics',
    vectors: [0.2, 0.3, 0.4]  // Mock embedding
  },
  {
    id: 'tow1',
    name: 'Rapid Towing',
    description: '24/7 emergency towing and roadside assistance',
    rating: 4.5,
    price: 95,
    image: '/images/tow1.jpg',
    location: '1.7 miles away',
    type: 'towing',
    vectors: [0.4, 0.5, 0.6]  // Mock embedding
  },
  {
    id: 'wash1',
    name: 'Sparkle Car Wash',
    description: 'Professional car washing and detailing services',
    rating: 4.3,
    price: 40,
    image: '/images/wash1.jpg',
    location: '5.2 miles away',
    type: 'carwash',
    vectors: [0.6, 0.7, 0.8]  // Mock embedding
  },
  {
    id: 'fuel1',
    name: 'GasNow Delivery',
    description: 'Emergency fuel delivery when you run out of gas',
    rating: 4.6,
    price: 50,
    image: '/images/fuel1.jpg',
    location: '0.9 miles away',
    type: 'fuel',
    vectors: [0.8, 0.9, 1.0]  // Mock embedding
  }
];

// Mock text embedding function - in production, this would use an actual embedding model
async function getTextEmbedding(text) {
  // Simplified mock - in production use OpenAI or other embedding service
  return [0.2, 0.3, 0.4]; // Mock 3-dimensional vector
}

// Simplified vector similarity function (cosine similarity)
function calculateCosineSimilarity(vecA, vecB) {
  if (vecA.length !== vecB.length) return 0;
  
  let dotProduct = 0;
  let normA = 0;
  let normB = 0;
  
  for (let i = 0; i < vecA.length; i++) {
    dotProduct += vecA[i] * vecB[i];
    normA += vecA[i] * vecA[i];
    normB += vecB[i] * vecB[i];
  }
  
  if (normA === 0 || normB === 0) return 0;
  return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB));
}

/**
 * Semantic search endpoint using Qdrant concepts
 */
export async function GET({ url }) {
  try {
    const query = url.searchParams.get('q') || '';
    const minRating = Number(url.searchParams.get('minRating')) || 0;
    const maxPrice = Number(url.searchParams.get('maxPrice')) || Infinity;
    const category = url.searchParams.get('category') || '';
    
    // In production, this would connect to Qdrant API
    // const QDRANT_URL = process.env.QDRANT_URL;
    // const QDRANT_API_KEY = process.env.QDRANT_API_KEY;
    
    // Get query embedding
    const queryEmbedding = await getTextEmbedding(query);
    
    // Filter and rank services
    let filteredServices = [...services];
    
    // Apply category filter if specified
    if (category) {
      filteredServices = filteredServices.filter(service => service.type === category);
    }
    
    // Apply rating and price filters
    filteredServices = filteredServices.filter(service => 
      service.rating >= minRating &&
      (!maxPrice || service.price <= maxPrice)
    );
    
    // In production with Qdrant, you would use their API:
    // const searchResults = await axios.post(`${QDRANT_URL}/collections/services/points/search`, {
    //   vector: queryEmbedding,
    //   limit: 10,
    //   filter: {
    //     must: [
    //       { key: 'rating', range: { gte: minRating } },
    //       ...(maxPrice ? [{ key: 'price', range: { lte: maxPrice } }] : []),
    //       ...(category ? [{ key: 'type', match: { value: category } }] : [])
    //     ]
    //   }
    // }, {
    //   headers: {
    //     'Content-Type': 'application/json',
    //     'API-Key': QDRANT_API_KEY
    //   }
    // });
    
    // For demo, calculate similarity scores manually
    const scoredServices = filteredServices.map(service => ({
      ...service,
      score: calculateCosineSimilarity(queryEmbedding, service.vectors)
    }));
    
    // Sort by relevance score
    const rankedResults = scoredServices
      .sort((a, b) => b.score - a.score)
      .map(({ vectors, score, ...service }) => service); // Remove vectors and score from response
    
    return json(rankedResults);
  } catch (error) {
    console.error('Search error:', error);
    return json({ error: 'Search failed' }, { status: 500 });
  }
} 
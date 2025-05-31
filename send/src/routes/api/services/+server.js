import { json } from '@sveltejs/kit';

// Mock data - in production, this would be retrieved from a database
const services = [
  {
    id: 'mech1',
    name: 'Quick Fix Auto',
    description: 'Fast and reliable car repair services for all makes and models',
    rating: 4.7,
    price: 80,
    image: '/images/mechanic1.jpg',
    location: '2.5 miles away',
    type: 'mechanics'
  },
  {
    id: 'mech2',
    name: 'Elite Car Care',
    description: 'Premium auto repair with certified technicians',
    rating: 4.9,
    price: 120,
    image: '/images/mechanic2.jpg',
    location: '3.8 miles away',
    type: 'mechanics'
  },
  {
    id: 'mech3',
    name: 'Precision Auto Shop',
    description: 'Specialized in foreign and luxury vehicles',
    rating: 4.6,
    price: 150,
    image: '/images/mechanic3.jpg',
    location: '4.1 miles away',
    type: 'mechanics'
  },
  {
    id: 'tow1',
    name: 'Rapid Towing',
    description: '24/7 emergency towing and roadside assistance',
    rating: 4.5,
    price: 95,
    image: '/images/tow1.jpg',
    location: '1.7 miles away',
    type: 'towing'
  },
  {
    id: 'tow2',
    name: 'Emergency Tow Services',
    description: 'Fast response time for all vehicle types',
    rating: 4.2,
    price: 85,
    image: '/images/tow2.jpg',
    location: '2.9 miles away',
    type: 'towing'
  },
  {
    id: 'wash1',
    name: 'Sparkle Car Wash',
    description: 'Professional car washing and detailing services',
    rating: 4.3,
    price: 40,
    image: '/images/wash1.jpg',
    location: '5.2 miles away',
    type: 'carwash'
  },
  {
    id: 'wash2',
    name: 'Deluxe Auto Detail',
    description: 'Premium detailing with ceramic coating options',
    rating: 4.8,
    price: 70,
    image: '/images/wash2.jpg',
    location: '3.5 miles away',
    type: 'carwash'
  },
  {
    id: 'fuel1',
    name: 'GasNow Delivery',
    description: 'Emergency fuel delivery when you run out of gas',
    rating: 4.6,
    price: 50,
    image: '/images/fuel1.jpg',
    location: '0.9 miles away',
    type: 'fuel'
  },
  {
    id: 'fuel2',
    name: 'Mobile Refueling',
    description: 'Scheduled fuel delivery for your convenience',
    rating: 4.4,
    price: 45,
    image: '/images/fuel2.jpg',
    location: '2.3 miles away',
    type: 'fuel'
  }
];

export function GET({ url }) {
  try {
    const type = url.searchParams.get('type') || 'mechanics';
    
    // Filter services by type
    const filteredServices = services.filter(service => service.type === type);
    
    return json(filteredServices);
  } catch (error) {
    console.error('Error retrieving services:', error);
    return json({ error: 'Failed to retrieve services' }, { status: 500 });
  }
} 
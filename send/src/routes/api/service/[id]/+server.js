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
    type: 'mechanics',
    hours: 'Mon-Sat: 8AM-7PM, Sun: 9AM-5PM',
    phone: '(555) 123-4567',
    address: '123 Auto Repair Lane, Carville',
    services: ['Oil Change', 'Brake Repair', 'Engine Diagnostics', 'Tire Replacement']
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
    hours: 'Mon-Fri: 7AM-8PM, Sat: 8AM-6PM, Sun: Closed',
    phone: '(555) 987-6543',
    address: '456 Car Service Road, Autoville',
    services: ['Engine Repair', 'Transmission Service', 'AC Repair', 'Electrical Systems']
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
    hours: '24/7 Emergency Service',
    phone: '(555) 777-8888',
    address: '789 Tow Truck Blvd, Towville',
    services: ['Local Towing', 'Long Distance Towing', 'Flatbed Service', 'Motorcycle Towing']
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
    hours: 'Mon-Sun: 7AM-9PM',
    phone: '(555) 456-7890',
    address: '321 Clean Car Ave, Sparkleton',
    services: ['Exterior Wash', 'Interior Cleaning', 'Waxing', 'Detailing']
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
    hours: 'Mon-Sun: 6AM-11PM',
    phone: '(555) 234-5678',
    address: '567 Fuel Lane, Gassville',
    services: ['Gas Delivery', 'Diesel Delivery', 'Emergency Refueling', 'Scheduled Delivery']
  }
];

export function GET({ params }) {
  try {
    const { id } = params;
    
    // Find service by ID
    const service = services.find(s => s.id === id);
    
    if (!service) {
      return json({ error: 'Service not found' }, { status: 404 });
    }
    
    return json(service);
  } catch (error) {
    console.error(`Error retrieving service ${params.id}:`, error);
    return json({ error: 'Failed to retrieve service details' }, { status: 500 });
  }
} 
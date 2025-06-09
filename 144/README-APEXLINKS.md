# ApexLinks Backend Implementation

This document outlines the implementation of the zones, users, products, and services features for the ApexLinks platform.

## Overview

The backend is built using the Warp web framework in Rust, with Qdrant as the vector database. All data is stored as vectors in Qdrant collections, using Google's embedding API for semantic search capabilities.

## Data Structure

### Tenant IDs
- `z` - Zones
- `u` - Users
- `p` - Products
- `s` - Services

### Common Fields
- `t` - Description
- `c` - Price (products/services)
- `u` - User ID (owner)
- `z` - Zone ID
- `l` - Location URL
- `p` - Position (lat/lng)
- `i` - Image URLs
- `n` - Name
- `s` - Tenant ID

## Features Implemented

### Zone Management
- Create zones with name, description, and location
- Check for similar zones to avoid duplication
- Search for zones by text query with embedding-based semantic search
- Optional location-based filtering with radius

### User Management
- Google OAuth2 authentication
- User profile with personal information
- Join/leave zones
- Create products and services

### Products and Services
- Create products/services that inherit zone, location, and position from user
- Search products/services with embedding-based semantic search
- Filter by zone, price range, etc.

## API Endpoints

### Authentication
- `GET /auth/google/login` - Initiate Google OAuth login
- `GET /auth/google/callback` - Handle OAuth callback

### Zone Management
- `POST /zone/add` - Create a new zone
- `POST /zone/search` - Search for zones
- `PUT /zone/edit/{id}` - Edit zone (placeholder)
- `DELETE /zone/delete/{id}` - Delete zone (placeholder)

### User Management
- `POST /user/search` - Search for users
- `POST /user/join_zone` - Join a zone
- `POST /user/leave_zone` - Leave a zone

### Product Management
- `POST /product/add` - Create a new product
- `POST /product/search` - Search for products
- `PUT /product/edit/{id}` - Edit product (placeholder)
- `DELETE /product/delete/{id}` - Delete product (placeholder)

### Service Management
- `POST /service/add` - Create a new service
- `POST /service/search` - Search for services
- `PUT /service/edit/{id}` - Edit service (placeholder)
- `DELETE /service/delete/{id}` - Delete service (placeholder)

## Technical Implementation Notes

### Vector Embeddings
All text data is converted to vector embeddings for semantic search using Google's embedding API. This enables natural language search across all entities.

### Inheritance
Products and services inherit zone, location, and position data from their creating user, reducing data duplication and ensuring consistency.

### IBM COS Integration
Image storage is prepared for IBM Cloud Object Storage, but the actual integration is a placeholder to be completed in the future.

### Authentication
Google OAuth2 is implemented for user authentication, with proper token exchange and profile retrieval.

## Future Enhancements
- Complete IBM COS integration for image storage
- Implement the edit and delete functionality for all entities
- Add authentication middleware to protect routes
- Add user sessions and JWT token authentication
- Add rate limiting to prevent abuse 
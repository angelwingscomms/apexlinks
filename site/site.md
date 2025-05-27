# ApexLinks Site Implementation

This document outlines the implementation of the ApexLinks site, a SvelteKit-based frontend application that connects to the 144 backend.

## Features Implemented

### User Authentication
- Google OAuth integration
- Login/logout functionality
- Authentication state management
- Secure session handling

### User Profile Management
- View user profile
- Edit profile information:
  - Username
  - Description
  - Age
  - Gender

### User Search
- Search for users by name, description or other attributes
- Filter by age and gender
- Display search results in an attractive UI

### User Similarity
- View compatibility metrics between users
- Display common interests and matching attributes
- Visualize similarity scores

### PWA Implementation
- Web app manifest
- Service worker for offline functionality
- Installable on mobile and desktop
- Responsive design across all devices

## Technology Stack

- **Frontend Framework**: SvelteKit 5
- **Styling**: Tailwind CSS v4 with custom utilities
- **State Management**: Svelte stores
- **HTTP Client**: Axios
- **Testing**: Vitest for unit tests, Playwright for E2E
- **PWA**: Custom service worker implementation

## Design Philosophy

The UI follows a neumorphic design philosophy with custom utilities in Tailwind CSS v4:

- Soft shadows and elevated elements
- Smooth animations and transitions
- Dreamy, ethereal color palette
- Responsive layouts for all devices

## File Structure

```
site/
├── src/
│   ├── lib/
│   │   ├── components/     # Reusable UI components
│   │   ├── services/       # API interaction services
│   │   └── stores/         # Svelte stores for state management
│   ├── routes/             # SvelteKit routes/pages
│   └── app.css             # Global styles with Tailwind v4
├── static/
│   ├── icons/              # PWA icons
│   ├── manifest.json       # Web App Manifest
│   └── service-worker.js   # Service worker for offline functionality
└── ...
```

## Future Enhancements

- Zone management (join/leave/create zones)
- User direct messaging
- Real-time notifications
- Enhanced profile customization
- More advanced search filters 
# Bible Verse Search Application

## Overview
This is a modern SvelteKit (Svelte 5) application for searching Bible verses. The application is entirely client-side using adapter-static and communicates directly with a Rust API for verse searching.

## Architecture
- **Frontend**: SvelteKit (Svelte 5) with Tailwind CSS for styling
- **Backend**: Rust API using warp for handling HTTP requests
- **Search Engine**: Google's Gemini model for semantic search capabilities
- **Database**: Qdrant vector database for storing and searching verse embeddings

## Features
- Search for Bible verses semantically (find verses that match the meaning, not just keywords)
- Filter search results by book and chapter
- Modern, minimalist UI with smooth animations
- Fully client-side rendering for fast performance

## API Integration
The application communicates with the Rust API through these endpoints:
- `/bible/search` - POST request for semantic search of Bible verses
  ```json
  {
    "query": "love your neighbor",
    "limit": 10,
    "book": 40,   // optional, filter by book number
    "chapter": 5  // optional, filter by chapter
  }
  ```

## Technology Stack
- **SvelteKit**: Modern web framework for building highly interactive UIs
- **Svelte 5**: Latest version with improved reactivity and performance
- **TailwindCSS**: Utility-first CSS framework for custom designs
- **Adapter-static**: For static site generation, making the app entirely client-side
- **Axios**: For HTTP requests to the Rust API

## Development
The application is designed to be highly responsive and provide a seamless user experience. Animation and transitions are implemented for a polished feel 
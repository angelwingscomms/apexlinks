# Bible Verse Embeddings with Gemini and Qdrant

This project creates semantic embeddings for Bible verses using Google's Gemini text-embedding-004 model and stores them in Qdrant vector database for semantic search.

## Prerequisites

- Rust and Cargo installed
- A Google Gemini API key
- Qdrant running and configured

## Setup

Create a `Secrets.toml` file in the project root with the following content:

```toml
GEMINI = "your_gemini_api_key"
QDRANT_URL = "your_qdrant_url" # e.g., http://localhost:6333
QDRANT_KEY = "your_qdrant_api_key" # if authentication is enabled
```

## Input Data Format

The tool expects a JSON file with Bible verses in the following format:

```json
[
  {
    "b": 1,     // book number
    "c": 1,     // chapter
    "v": 1,     // verse
    "t": "In the beginning God created the heavens and the earth"  // text
  },
  // ... more verses
]
```

## API Routes

The following API routes are available for working with Bible verse embeddings:

### Embedding Verses

```
GET /bible/embed
```

This is a helper route that automatically reads verses from `ylt.json` in the application's root directory and embeds them in Qdrant. No request body is needed.

Response:
```json
{
  "status": "success",
  "count": 31102  // Total number of verses processed
}
```

### Searching for Similar Verses

```
POST /bible/search
```

Request body:
```json
{
  "query": "love your neighbor",
  "limit": 5
}
```

The `limit` field is optional and defaults to 5.

Response:
```json
[
  {
    "book": 40,
    "chapter": 22,
    "verse": 39,
    "text": "And the second is like unto it, Thou shalt love thy neighbour as thyself.",
    "reference": "40 22:39"
  },
  // ... more results
]
```

## Command Line Usage

In addition to the API routes, command-line tools are provided for batch processing and testing.

### Embedding Bible Verses

To process and embed all verses from the YLT JSON file:

```bash
cargo run --bin embed_bible /path/to/ylt.json
```

If no path is specified, it will look for `ylt.json` in the current directory.

### Searching for Similar Verses

To search for verses semantically similar to a query:

```bash
cargo run --bin search_verses "love your neighbor" 10
```

The first argument is your search query, and the second (optional) argument is the number of results to return (defaults to 5).

## How It Works

1. The `embed_bible` binary and `/bible/embed` route:
   - Read Bible verses from the YLT JSON file
   - Create a collection in Qdrant if it doesn't exist
   - Process verses in batches
   - For each verse, generate an embedding using Gemini's text-embedding-004 model
   - Store the embedding along with verse metadata in Qdrant

2. The `search_verses` binary and `/bible/search` route:
   - Take a search query from command line arguments or request payload
   - Generate an embedding for the query using the same model
   - Search Qdrant for semantically similar verses
   - Return the results

## Technical Implementation

- Uses the Gemini text-embedding-004 model which provides 768-dimensional embeddings
- Implements batch processing with rate limiting to respect API quotas
- Stores metadata alongside embeddings for easy retrieval
- Uses cosine similarity for semantic searching
- Reads configuration from Secrets.toml for secure API key management

## Dependencies

- reqwest: HTTP client for API requests
- serde/serde_json: For JSON serialization/deserialization
- tokio: Asynchronous runtime
- uuid: For generating unique IDs
- warp: Web framework for API routes
- toml: For parsing the Secrets.toml configuration file 
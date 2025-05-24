# RedMoon - AI-Powered Anonymous Chat

RedMoon is an Omegle-style anonymous chat application that uses AI-powered matching to connect users based on their interests and descriptions. The system uses Google's Gemini embedding model to create semantic embeddings of user descriptions and stores them in Qdrant vector database for intelligent matching.

## Features

- **AI-Powered Matching**: Uses Gemini embeddings to match users based on semantic similarity of their descriptions
- **Real-time Chat**: WebSocket-based real-time messaging
- **Anonymous**: No registration required, users are matched anonymously
- **Dark Cyberpunk Theme**: Beautiful dark UI with red accents and cyberpunk aesthetics
- **Interest-based Filtering**: Users can specify interests for better matching
- **Age Range Preferences**: Optional age range filtering

## Architecture

### Backend (144/)
- **Language**: Rust
- **Framework**: Warp web framework
- **Database**: Qdrant vector database for embeddings
- **AI**: Google Gemini embedding model
- **WebSockets**: Real-time chat communication

### Frontend (redmoon/)
- **Framework**: SvelteKit
- **Styling**: Tailwind CSS + DaisyUI
- **HTTP Client**: Axios
- **Theme**: Dark cyberpunk with red accents

## Setup

### Prerequisites
- Rust (latest stable)
- Shuttle CLI (`cargo install cargo-shuttle`)
- Node.js (v18+)
- Qdrant database running locally or remotely
- Google Gemini API key

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd 144
   ```

2. Install Shuttle CLI and dependencies:
   ```bash
   # Install Shuttle CLI if you haven't already
   cargo install cargo-shuttle
   
   # Build dependencies
   cargo build
   ```

3. Set up your secrets in `Secrets.toml`:
   ```toml
   GEMINI_API_KEY = "your_gemini_api_key_here"
   QDRANT_URL = "http://localhost:6333"  # or your Qdrant instance URL
   ```

4. Start Qdrant (if running locally):
   ```bash
   # Using Docker
   docker run -p 6333:6333 qdrant/qdrant
   ```

5. Run the backend:
   ```bash
   shuttle run
   ```

The backend will start on `http://localhost:8000` (Shuttle's default local port)

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd redmoon
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the development server:
   ```bash
   npm run dev
   ```

The frontend will start on `http://localhost:5173`

## Usage

1. Open your browser and go to `http://localhost:5173`
2. Fill in your description (what you're interested in talking about)
3. Add comma-separated interests
4. Optionally select an age range
5. Click "Find Match" to start searching
6. Once matched, you'll be connected to a chat session
7. Start chatting with your matched partner!

## API Endpoints

### POST /chat/match
Find a compatible chat partner based on user description and interests.

**Request Body:**
```json
{
  "description": "I love discussing technology and programming",
  "interests": ["programming", "technology", "AI"],
  "age_range": "25-35"
}
```

**Response:**
```json
{
  "match_found": true,
  "session_id": "session_uuid",
  "partner_id": "partner_uuid",
  "message": "Match found! You can start chatting."
}
```

### WebSocket /chat/ws
Real-time chat communication.

**Message Types:**
- `join_session`: Join a chat session
- `chat`: Send a chat message
- `leave_session`: Leave a chat session

## How Matching Works

1. **Embedding Creation**: When a user requests a match, their description and interests are combined and sent to Google's Gemini embedding model
2. **Vector Storage**: The resulting embedding vector is stored in Qdrant vector database
3. **Similarity Search**: The system searches for similar users using cosine similarity
4. **Threshold Matching**: Users with similarity above 0.7 are considered compatible matches
5. **Session Creation**: When a match is found, a chat session is created and both users are connected

## Deployment

This project uses [Shuttle](https://shuttle.rs) for easy deployment to the cloud.

### Deploy to Production
```bash
cd 144
shuttle deploy
```

### Local Development with Shuttle
```bash
cd 144
shuttle run  # Starts the server locally on port 8000
```

## Development

### Backend Structure
```
144/src/
├── routes/
│   └── chat/
│       ├── mod.rs          # Route definitions
│       ├── types.rs        # Data structures
│       ├── matching.rs     # Matching logic
│       └── websocket.rs    # WebSocket handling
├── gemini_embed.rs         # Gemini API integration
└── main.rs                 # Server setup
```

### Frontend Structure
```
redmoon/src/
├── routes/
│   ├── +layout.svelte      # Layout component
│   └── +page.svelte        # Main chat interface
├── app.html                # HTML template
└── app.css                 # Global styles
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is open source and available under the MIT License.

## Security Note

This is a demonstration project. For production use, consider implementing:
- Rate limiting
- Content moderation
- User reporting system
- Enhanced security measures
- Proper error handling and logging 